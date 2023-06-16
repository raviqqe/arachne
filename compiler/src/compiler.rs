use crate::{frame::Frame, CompileError};
use async_stream::try_stream;
use futures::{Stream, StreamExt};
use runtime::{Array, Symbol, TypedValueRef, Value};
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
    ) -> Result<(), CompileError> {
        match Array::try_from(value) {
            Ok(array) => {
                if let Some(symbol) = array.get_usize(0).to_symbol() {
                    match symbol.as_str() {
                        "let" => {
                            if let Some(symbol) = array.get_usize(1).to_symbol() {
                                self.compile_expression(array.get_usize(2), frame)?;
                                frame.insert_variable(symbol);
                                *frame.temporary_count_mut() -= 1;
                            }
                        }
                        "let-rec" => {
                            if let (Some(symbol), Some(array)) = (
                                array.get_usize(1).to_symbol(),
                                array.get_usize(2).as_array(),
                            ) {
                                self.compile_function(Some(symbol), array, frame)?;
                                frame.insert_variable(symbol);
                                *frame.temporary_count_mut() -= 1;
                            }
                        }
                        _ => self.compile_expression_statement(array.into(), frame, dump)?,
                    }
                } else {
                    self.compile_expression_statement(array.into(), frame, dump)?
                }
            }
            Err(value) => self.compile_expression_statement(value, frame, dump)?,
        }

        Ok(())
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
        if let Some(value) = value.as_typed() {
            match value {
                TypedValueRef::Array(array) => {
                    if let Some(symbol) = array.get_usize(0).to_symbol() {
                        let symbol = symbol.as_str();

                        if symbol == "fn" {
                            self.compile_function(None, array, frame)?;
                        } else if symbol == "if" {
                            self.compile_if(array, 1, frame)?;
                        } else if let Some(instruction) = match symbol {
                            "=" => Some(Instruction::Equal),
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
                TypedValueRef::Closure(_) => return Err(CompileError::Closure),
                TypedValueRef::Float64(number) => {
                    let mut codes = self.codes.borrow_mut();

                    codes.push(Instruction::Float64 as u8);
                    codes.extend(number.to_f64().to_le_bytes());
                    *frame.temporary_count_mut() += 1;
                }
                TypedValueRef::Integer32(number) => {
                    let mut codes = self.codes.borrow_mut();

                    codes.push(Instruction::Integer32 as u8);
                    codes.extend(number.to_i32().to_le_bytes());
                    *frame.temporary_count_mut() += 1;
                }
                TypedValueRef::Symbol(symbol) => {
                    let mut codes = self.codes.borrow_mut();

                    if let Some(index) = frame.get_variable(symbol) {
                        codes.push(Instruction::Peek as u8);
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

    fn compile_function(
        &mut self,
        name: Option<Symbol>,
        array: &Array,
        frame: &mut Frame,
    ) -> Result<(), CompileError> {
        let mut codes = self.codes.borrow_mut();

        codes.push(Instruction::Jump as u8);
        codes.extend(0u16.to_le_bytes()); // stub address
        let jump_index = codes.len();

        let function_index = codes.len();
        drop(codes);

        let arguments = array.get_usize(1);
        let arguments = arguments.as_array().expect("arguments");
        let arity = u8::try_from(arguments.len_usize())?;

        let closed_frame = {
            let mut frame = Frame::with_capacity(arguments.len_usize() + 1);

            if let Some(name) = name {
                frame.insert_variable(name);
            }

            for index in 0..arguments.len_usize() {
                if let Some(argument) = arguments.get_usize(index).to_symbol() {
                    frame.insert_variable(argument);
                }
            }

            for index in 2..array.len_usize() - 1 {
                self.compile_statement(array.get_usize(index), &mut frame, false)?;
            }

            self.compile_expression(array.get_usize(array.len_usize() - 1), &mut frame)?;

            let mut codes = self.codes.borrow_mut();

            codes.push(Instruction::Return as u8);
            *frame.temporary_count_mut() -= 1;
            assert_eq!(*frame.temporary_count_mut(), 0);

            frame
        };

        let mut codes = self.codes.borrow_mut();
        let current_index = codes.len();

        codes[jump_index - size_of::<u16>()..jump_index]
            .copy_from_slice(&((current_index - jump_index) as u16).to_le_bytes());

        codes.push(Instruction::Close as u8);
        codes.extend((function_index as u32).to_le_bytes());
        codes.push(arity); // arity
        codes.push(closed_frame.free_variables().len() as u8);
        dbg!(&closed_frame.free_variables());

        for &name in &*closed_frame.free_variables() {
            codes.push(frame.get_variable(name).expect("existing variable") as u8);
        }

        *frame.temporary_count_mut() += 1;

        Ok(())
    }

    fn compile_arguments(&mut self, array: &Array, frame: &mut Frame) -> Result<(), CompileError> {
        for index in 1..array.len_usize() {
            self.compile_expression(array.get_usize(index), frame)?;
        }

        Ok(())
    }

    fn compile_call(&mut self, array: &Array, frame: &mut Frame) -> Result<(), CompileError> {
        self.compile_expression(array.get_usize(0), frame)?;
        self.compile_arguments(array, frame)?;

        let mut codes = self.codes.borrow_mut();
        codes.push(Instruction::Call as u8);
        codes.push((array.len_usize() - 1) as u8);
        *frame.temporary_count_mut() -= array.len_usize() - 1;

        Ok(())
    }

    fn compile_if(
        &mut self,
        array: &Array,
        condition_index: usize,
        frame: &mut Frame,
    ) -> Result<(), CompileError> {
        self.compile_expression(array.get_usize(condition_index), frame)?;

        let mut codes = self.codes.borrow_mut();
        codes.push(Instruction::Branch as u8);
        codes.extend(0u16.to_le_bytes());
        let branch_index = codes.len();
        drop(codes);
        *frame.temporary_count_mut() -= 1;

        let else_index = {
            let mut frame = frame.block();

            if condition_index + 3 < array.len_usize() {
                self.compile_if(array, condition_index + 2, &mut frame)?;
            } else {
                self.compile_expression(array.get_usize(condition_index + 2), &mut frame)?;
            }

            let mut codes = self.codes.borrow_mut();
            codes.push(Instruction::Jump as u8);
            codes.extend(0u16.to_le_bytes());
            codes.len()
        };

        {
            let mut codes = self.codes.borrow_mut();
            let current_index = codes.len();
            codes[branch_index - size_of::<u16>()..branch_index]
                .copy_from_slice(&((current_index - branch_index) as i16).to_le_bytes());
            drop(codes);

            let mut frame = frame.block();
            self.compile_expression(array.get_usize(condition_index + 1), &mut frame)?;
        }

        let mut codes = self.codes.borrow_mut();
        let current_index = codes.len();
        codes[else_index - size_of::<u16>()..else_index]
            .copy_from_slice(&((current_index - else_index) as i16).to_le_bytes());

        *frame.temporary_count_mut() += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{pin_mut, stream::iter};
    use std::io;
    use vm::format_instructions;

    type Error = io::Error;

    async fn compile<const N: usize>(values: [Value; N]) -> String {
        let codes = vec![].into();
        let mut compiler = Compiler::new(&codes);
        let values = iter(values).map(Ok);

        pin_mut!(values);

        let results = compiler.compile::<Error>(&mut values);

        pin_mut!(results);

        while let Some(result) = results.next().await {
            result.unwrap();
        }

        let instructions = format_instructions(&codes.borrow()).unwrap();

        instructions
    }

    #[tokio::test]
    async fn compile_symbol() {
        insta::assert_display_snapshot!(compile(["foo".into()]).await);
    }

    mod function {
        use super::*;

        #[tokio::test]
        async fn compile_function_with_zero_argument() {
            insta::assert_display_snapshot!(
                compile([["fn".into(), [].into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_function_with_one_argument() {
            insta::assert_display_snapshot!(
                compile([["fn".into(), ["x".into()].into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_function_with_two_arguments() {
            insta::assert_display_snapshot!(
                compile([["fn".into(), ["x".into(), "y".into()].into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_function_with_let() {
            insta::assert_display_snapshot!(
                compile([[
                    "fn".into(),
                    ["x".into()].into(),
                    ["let".into(), "y".into(), "x".into()].into(),
                    "y".into()
                ]
                .into()])
                .await
            );
        }

        #[tokio::test]
        async fn compile_function_with_two_let() {
            insta::assert_display_snapshot!(
                compile([[
                    "fn".into(),
                    ["x".into()].into(),
                    ["let".into(), "y".into(), "x".into()].into(),
                    ["let".into(), "z".into(), "y".into()].into(),
                    "z".into()
                ]
                .into()])
                .await
            );
        }
    }

    mod r#if {
        use super::*;

        #[tokio::test]
        async fn compile_if() {
            insta::assert_display_snapshot!(
                compile([["if".into(), 1.0.into(), 42.0.into(), 13.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_without_else() {
            insta::assert_display_snapshot!(
                compile([["if".into(), 1.0.into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_two_branches() {
            insta::assert_display_snapshot!(
                compile([[
                    "if".into(),
                    1.0.into(),
                    2.0.into(),
                    3.0.into(),
                    4.0.into(),
                    5.0.into()
                ]
                .into()])
                .await
            );
        }

        #[tokio::test]
        async fn compile_two_branches_without_else() {
            insta::assert_display_snapshot!(
                compile([["if".into(), 1.0.into(), 2.0.into(), 3.0.into(), 4.0.into()].into()])
                    .await
            );
        }

        #[tokio::test]
        async fn compile_two_branches_in_function() {
            insta::assert_display_snapshot!(
                compile([[
                    "let".into(),
                    "f".into(),
                    [
                        "fn".into(),
                        [].into(),
                        [
                            "if".into(),
                            1.0.into(),
                            2.0.into(),
                            3.0.into(),
                            4.0.into(),
                            5.0.into()
                        ]
                        .into()
                    ]
                    .into()
                ]
                .into()])
                .await
            );
        }
    }

    mod r#let {
        use super::*;

        #[tokio::test]
        async fn compile_let() {
            insta::assert_display_snapshot!(
                compile([["let".into(), "x".into(), 42.0.into()].into()]).await
            );
        }

        #[tokio::test]
        async fn compile_two_let() {
            insta::assert_display_snapshot!(
                compile([
                    ["let".into(), "x".into(), 42.0.into()].into(),
                    ["let".into(), "y".into(), "x".into()].into(),
                ])
                .await
            );
        }

        #[tokio::test]
        async fn compile_two_let_with_same_name() {
            insta::assert_display_snapshot!(
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
            insta::assert_display_snapshot!(
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
            insta::assert_display_snapshot!(
                compile([
                    ["let".into(), "x".into(), 1.0.into()].into(),
                    ["let".into(), "y".into(), 2.0.into()].into(),
                    ["let".into(), "z".into(), "x".into()].into(),
                ])
                .await
            );
        }
    }

    mod let_rec {
        use super::*;

        #[tokio::test]
        async fn compile_let() {
            insta::assert_display_snapshot!(
                compile([[
                    "let-rec".into(),
                    "f".into(),
                    ["fn".into(), [].into(), ["f".into()].into()].into()
                ]
                .into()])
                .await
            );
        }
    }
}
