use crate::CompileError;
use async_stream::try_stream;
use futures::{Stream, StreamExt};
use runtime::{Array, Symbol, TypedValue, Value};
use std::{cell::RefCell, collections::HashMap, error::Error, mem::size_of};
use vm::Instruction;

const GLOBAL_VARIABLE_CAPACITY: usize = 1 << 8;

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
            let mut global_variables = HashMap::with_capacity(GLOBAL_VARIABLE_CAPACITY);

            while let Some(value) = values.next().await {
                self.compile_statement(value.map_err(|error| CompileError::Other(error.into()))?, &mut global_variables, true)?;
                yield ();
            }
        }
    }

    fn compile_statement(
        &mut self,
        value: Value,
        variables: &mut HashMap<Symbol, usize>,
        dump: bool,
    ) -> Result<bool, CompileError> {
        Ok(match Array::try_from(value) {
            Ok(array) => {
                if let Some(symbol) = array.get_usize(0).to_symbol() {
                    match symbol.as_str() {
                        "let" => {
                            if let Some(symbol) = array.get_usize(1).to_symbol() {
                                self.compile_expression(array.get_usize(2), variables)?;
                                variables.insert(symbol, variables.len());
                                // Keep a value on a stack.

                                true
                            } else {
                                false
                            }
                        }
                        _ => {
                            self.compile_expression_statement(array.into(), variables, dump)?;
                            false
                        }
                    }
                } else {
                    self.compile_expression_statement(array.into(), variables, dump)?;
                    false
                }
            }
            Err(value) => {
                self.compile_expression_statement(value, variables, dump)?;
                false
            }
        })
    }

    fn compile_expression_statement(
        &mut self,
        value: Value,
        variables: &mut HashMap<Symbol, usize>,
        dump: bool,
    ) -> Result<(), CompileError> {
        self.compile_expression(value, variables)?;

        if dump {
            self.codes.borrow_mut().push(Instruction::Dump as u8);
        }

        self.codes.borrow_mut().push(Instruction::Drop as u8);

        Ok(())
    }

    fn compile_expression(
        &mut self,
        value: Value,
        variables: &mut HashMap<Symbol, usize>,
    ) -> Result<(), CompileError> {
        if let Some(value) = value.into_typed() {
            match value {
                TypedValue::Array(array) => {
                    if let Some(symbol) = array.get_usize(0).to_symbol() {
                        let symbol = symbol.as_str();

                        if symbol == "fn" {
                            let mut codes = self.codes.borrow_mut();

                            codes.push(Instruction::Jump as u8);
                            let jump_target_index = codes.len();
                            codes.extend(0u32.to_le_bytes()); // stub address

                            let function_index = codes.len();
                            let arguments = array.get_usize(1);
                            let arguments = arguments.as_array().expect("arguments");
                            let arity = u8::try_from(arguments.len_usize())?;
                            let mut frame_size = arity;

                            let mut variables = HashMap::with_capacity(arguments.len_usize());

                            for index in 0..arguments.len_usize() {
                                if let Some(argument) = arguments.get_usize(index).to_symbol() {
                                    variables.insert(argument, index);
                                }
                            }

                            for index in 0..array.len_usize() - 2 {
                                if self.compile_statement(
                                    array.get_usize(index),
                                    &mut variables,
                                    false,
                                )? {
                                    frame_size += 1
                                };
                            }

                            codes.push(Instruction::Return as u8);
                            codes.push(frame_size);

                            let current_index = codes.len();

                            codes[jump_target_index..jump_target_index + size_of::<u32>()]
                                .copy_from_slice(&(current_index as u32).to_le_bytes());

                            codes.push(Instruction::Close as u8);
                            codes.extend((function_index as u32).to_le_bytes());
                            codes.push(arity); // arity
                            codes.push(0u8); // TODO environment size
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
                            self.compile_arguments(array, variables)?;
                            self.codes.borrow_mut().push(instruction as u8);
                        } else {
                            self.compile_call(array, variables)?;
                        }
                    } else {
                        self.compile_call(array, variables)?
                    }
                }
                TypedValue::Closure(_) => return Err(CompileError::Closure),
                TypedValue::Float64(number) => {
                    self.codes.borrow_mut().push(Instruction::Float64 as u8);
                    self.codes
                        .borrow_mut()
                        .extend(number.to_f64().to_le_bytes());
                }
                TypedValue::Symbol(symbol) => {
                    let mut codes = self.codes.borrow_mut();

                    if let Some(&index) = variables.get(&symbol) {
                        codes.push(Instruction::Local as u8);
                        codes.push(index as u8);
                    } else if symbol.as_str().len() >= 1 << 8 {
                        return Err(CompileError::SymbolLength(symbol.as_str().into()));
                    } else {
                        codes.push(Instruction::Symbol as u8);
                        codes.push(symbol.as_str().len() as u8);
                        codes.extend(symbol.as_str().as_bytes());
                    }
                }
            }
        } else {
            self.codes.borrow_mut().push(Instruction::Nil as u8);
        }

        Ok(())
    }

    fn compile_arguments(
        &mut self,
        array: Array,
        variables: &mut HashMap<Symbol, usize>,
    ) -> Result<(), CompileError> {
        for index in 1..array.len_usize() {
            self.compile_expression(array.get_usize(index), variables)?;
        }

        Ok(())
    }

    fn compile_call(
        &mut self,
        array: Array,
        variables: &mut HashMap<Symbol, usize>,
    ) -> Result<(), CompileError> {
        self.compile_arguments(array.clone(), variables)?;
        self.compile_expression(array.get_usize(0), variables)?;
        self.codes.borrow_mut().push(Instruction::Call as u8);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{pin_mut, FutureExt};
    use std::io;
    use vm::decode_instructions;

    type Error = io::Error;

    #[tokio::test]
    async fn compile_symbol() {
        let codes = vec![].into();
        let mut compiler = Compiler::new(&codes);
        let values = async { Ok("foo".into()) }.into_stream();

        pin_mut!(values);

        let results = compiler.compile::<Error>(&mut values);

        pin_mut!(results);

        while let Some(result) = results.next().await {
            result.unwrap();
        }

        insta::assert_debug_snapshot!(decode_instructions(&codes.borrow()).unwrap());
    }
}
