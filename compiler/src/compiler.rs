use crate::{frame::Frame, CompileError};
use async_stream::try_stream;
use futures::{Stream, StreamExt};
use runtime::{Array, TypedValue, Value};
use std::{cell::RefCell, error::Error, mem::size_of};
use vm::Instruction;

pub struct Compiler<'a> {
    codes: &'a RefCell<Vec<u8>>,
}

impl<'a> Compiler<'a> {
    pub fn new(codes: &'a RefCell<Vec<u8>>) -> Self {
        Self { codes }
    }

    pub fn compile<E: Error + 'static>(
        &'a mut self,
        values: &'a mut (impl Stream<Item = Result<Value, E>> + Unpin),
    ) -> impl Stream<Item = Result<(), CompileError>> + 'a {
        try_stream! {
            let mut frame = Frame::new();

            while let Some(value) = values.next().await {
                self.compile_statement(value.map_err(|error| CompileError::Other(error.into()))?, &mut frame, true)?;
                yield ();
            }
        }
    }

    fn compile_statement(
        &mut self,
        value: Value,
        frame: &mut Frame,
        dump: bool,
    ) -> Result<bool, CompileError> {
        Ok(match Array::try_from(value) {
            Ok(array) => {
                if let Some(symbol) = array.get_usize(0).to_symbol() {
                    match symbol.as_str() {
                        "let" => {
                            if let Some(symbol) = array.get_usize(1).to_symbol() {
                                self.compile_expression(array.get_usize(2), frame)?;
                                frame.insert_variable(symbol);
                                *frame.temporary_count_mut() -= 1;

                                true
                            } else {
                                false
                            }
                        }
                        _ => {
                            self.compile_expression_statement(array.into(), frame, dump)?;
                            false
                        }
                    }
                } else {
                    self.compile_expression_statement(array.into(), frame, dump)?;
                    false
                }
            }
            Err(value) => {
                self.compile_expression_statement(value, frame, dump)?;
                false
            }
        })
    }

    fn compile_expression_statement(
        &mut self,
        value: Value,
        frame: &mut Frame,
        dump: bool,
    ) -> Result<(), CompileError> {
        self.compile_expression(value, frame)?;
        let mut codes = self.codes.borrow_mut();

        if dump {
            codes.push(Instruction::Dump as u8);
        }

        codes.push(Instruction::Drop as u8);
        *frame.temporary_count_mut() -= 1;

        Ok(())
    }

    fn compile_expression(&mut self, value: Value, frame: &mut Frame) -> Result<(), CompileError> {
        if let Some(value) = value.into_typed() {
            match value {
                TypedValue::Array(array) => {
                    if let Some(symbol) = array.get_usize(0).to_symbol() {
                        let symbol = symbol.as_str();

                        if symbol == "fn" {
                            let mut codes = self.codes.borrow_mut();

                            let jump_index = codes.len();
                            codes.push(Instruction::Jump as u8);
                            codes.extend(0u16.to_le_bytes()); // stub address

                            let function_index = codes.len();
                            drop(codes);

                            let arguments = array.get_usize(1);
                            let arguments = arguments.as_array().expect("arguments");
                            let arity = u8::try_from(arguments.len_usize())?;

                            {
                                let mut frame_size = arity;

                                let mut frame = Frame::with_capacity(arguments.len_usize());

                                for index in 0..arguments.len_usize() {
                                    if let Some(argument) = arguments.get_usize(index).to_symbol() {
                                        frame.insert_variable(argument);
                                    }
                                }

                                for index in 2..array.len_usize() - 1 {
                                    if self.compile_statement(
                                        array.get_usize(index),
                                        &mut frame,
                                        false,
                                    )? {
                                        frame_size += 1
                                    };
                                }

                                self.compile_expression(
                                    array.get_usize(array.len_usize() - 1),
                                    &mut frame,
                                )?;

                                let mut codes = self.codes.borrow_mut();

                                codes.push(Instruction::Return as u8);
                                codes.push(frame_size);
                                *frame.temporary_count_mut() -= 1;
                                assert_eq!(*frame.temporary_count_mut(), 0);
                            }

                            let mut codes = self.codes.borrow_mut();
                            let current_index = codes.len();

                            codes[jump_index + 1..jump_index + 1 + size_of::<u16>()]
                                .copy_from_slice(
                                    &((current_index - jump_index) as u16).to_le_bytes(),
                                );

                            codes.push(Instruction::Close as u8);
                            codes.extend((function_index as u32).to_le_bytes());
                            codes.push(arity); // arity
                            codes.push(0u8); // TODO environment size
                            *frame.temporary_count_mut() += 1;
                        } else if let Some(instruction) = match symbol {
                            "array" => Some(Instruction::Array),
                            "eq" => Some(Instruction::Equal),
                            "get" => Some(Instruction::Get),
                            "set" => Some(Instruction::Set),
                            "len" => Some(Instruction::Length),
                            "+" => Some(Instruction::Add),
                            "-" => Some(Instruction::Subtract),
                            "*" => Some(Instruction::Multiply),
                            "/" => Some(Instruction::Divide),
                            _ => None,
                        } {
                            self.compile_arguments(array, frame)?;
                            self.codes.borrow_mut().push(instruction as u8);
                            *frame.temporary_count_mut() -= match instruction {
                                Instruction::Array => todo!(),
                                Instruction::Length => 0,
                                Instruction::Set => 2,
                                _ => 1,
                            };
                        } else {
                            self.compile_call(array, frame)?;
                        }
                    } else {
                        self.compile_call(array, frame)?
                    }
                }
                TypedValue::Closure(_) => return Err(CompileError::Closure),
                TypedValue::Float64(number) => {
                    let mut codes = self.codes.borrow_mut();

                    codes.push(Instruction::Float64 as u8);
                    codes.extend(number.to_f64().to_le_bytes());
                    *frame.temporary_count_mut() += 1;
                }
                TypedValue::Symbol(symbol) => {
                    let mut codes = self.codes.borrow_mut();

                    if let Some(index) = frame.get_variable(symbol) {
                        codes.push(Instruction::Local as u8);
                        codes.push(index as u8);
                        *frame.temporary_count_mut() += 1;
                    } else if symbol.as_str().len() >= 1 << 8 {
                        return Err(CompileError::SymbolLength(symbol.as_str().into()));
                    } else {
                        codes.push(Instruction::Symbol as u8);
                        codes.push(symbol.as_str().len() as u8);
                        codes.extend(symbol.as_str().as_bytes());
                        *frame.temporary_count_mut() += 1;
                    }
                }
            }
        } else {
            self.codes.borrow_mut().push(Instruction::Nil as u8);
            *frame.temporary_count_mut() += 1;
        }

        Ok(())
    }

    fn compile_arguments(&mut self, array: Array, frame: &mut Frame) -> Result<(), CompileError> {
        for index in 1..array.len_usize() {
            self.compile_expression(array.get_usize(index), frame)?;
        }

        Ok(())
    }

    fn compile_call(&mut self, array: Array, frame: &mut Frame) -> Result<(), CompileError> {
        self.compile_arguments(array.clone(), frame)?;
        self.compile_expression(array.get_usize(0), frame)?;

        self.codes.borrow_mut().push(Instruction::Call as u8);
        *frame.temporary_count_mut() += 1 - array.len_usize();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{pin_mut, stream::iter};
    use std::io;
    use vm::{decode_instructions, InstructionIr};

    type Error = io::Error;

    async fn compile<const N: usize>(values: [Value; N]) -> Vec<InstructionIr> {
        let codes = vec![].into();
        let mut compiler = Compiler::new(&codes);
        let values = iter(values).map(Ok);

        pin_mut!(values);

        let results = compiler.compile::<Error>(&mut values);

        pin_mut!(results);

        while let Some(result) = results.next().await {
            result.unwrap();
        }

        let instructions = decode_instructions(&codes.borrow()).unwrap();

        instructions
    }

    #[tokio::test]
    async fn compile_symbol() {
        insta::assert_debug_snapshot!(compile(["foo".into()]).await);
    }

    mod function {
        use super::*;

        #[tokio::test]
        async fn compile_function_with_zero_argument() {
            insta::assert_debug_snapshot!(
                compile([["fn".into(), [].into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_function_with_one_argument() {
            insta::assert_debug_snapshot!(
                compile([["fn".into(), ["x".into()].into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_function_with_two_arguments() {
            insta::assert_debug_snapshot!(
                compile([["fn".into(), ["x".into(), "y".into()].into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_function_with_let() {
            insta::assert_debug_snapshot!(
                compile([["fn".into(), ["x".into()].into(), 42.0.into()].into()]).await
            );
        }
    }

    mod r#let {
        use super::*;

        #[tokio::test]
        async fn compile_let() {
            insta::assert_debug_snapshot!(
                compile([["let".into(), "x".into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_two_let() {
            insta::assert_debug_snapshot!(
                compile([
                    ["let".into(), "x".into(), 42.0.into()].into(),
                    ["let".into(), "y".into(), "x".into()].into(),
                ])
                .await
            );
        }

        #[tokio::test]
        async fn compile_two_let_with_same_name() {
            insta::assert_debug_snapshot!(
                compile([
                    ["let".into(), "x".into(), 42.0.into()].into(),
                    ["let".into(), "x".into(), 2045.0.into()].into(),
                    ["let".into(), "y".into(), "x".into()].into(),
                ])
                .await
            );
        }

        #[tokio::test]
        async fn compile_three_let() {
            insta::assert_debug_snapshot!(
                compile([
                    ["let".into(), "x".into(), 42.0.into()].into(),
                    ["let".into(), "y".into(), "x".into()].into(),
                    ["let".into(), "z".into(), "y".into()].into(),
                ])
                .await
            );
        }

        #[tokio::test]
        async fn compile_three_let_referencing_old() {
            insta::assert_debug_snapshot!(
                compile([
                    ["let".into(), "x".into(), 1.0.into()].into(),
                    ["let".into(), "y".into(), 2.0.into()].into(),
                    ["let".into(), "z".into(), "x".into()].into(),
                ])
                .await
            );
        }
    }
}
