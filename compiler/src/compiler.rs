use async_stream::try_stream;
use futures::{Stream, StreamExt};
use runtime::{Array, Symbol, TypedValue, Value};
use std::{cell::RefCell, collections::HashMap, error::Error, mem::size_of};
use vm::Instruction;

const VARIABLE_CAPACITY: usize = 1 << 8;

pub struct Compiler<'a> {
    codes: &'a RefCell<Vec<u8>>,
    variables: HashMap<Symbol, usize>,
}

impl<'a> Compiler<'a> {
    pub fn new(codes: &'a RefCell<Vec<u8>>) -> Self {
        Self {
            codes,
            variables: HashMap::with_capacity(VARIABLE_CAPACITY),
        }
    }

    pub fn compile<E: Error + 'static>(
        &'a mut self,
        values: &'a mut (impl Stream<Item = Result<Value, E>> + Unpin),
    ) -> impl Stream<Item = Result<(), E>> + 'a {
        try_stream! {
            while let Some(value) = values.next().await {
                self.compile_statement(value?);
                yield ();
            }
        }
    }

    fn compile_statement(&mut self, value: Value) {
        match Array::try_from(value) {
            Ok(array) => {
                if let Some(symbol) = array.get_usize(0).to_symbol() {
                    match symbol.as_str() {
                        "let" => {
                            if let Some(symbol) = array.get_usize(1).to_symbol() {
                                self.compile_expression(array.get_usize(2));
                                self.variables.insert(symbol, self.variables.len());
                                // Keep a value on a stack.
                            }
                        }
                        _ => self.compile_top_expression(array.into()),
                    }
                } else {
                    self.compile_top_expression(array.into());
                }
            }
            Err(value) => self.compile_top_expression(value),
        }
    }

    fn compile_top_expression(&mut self, value: Value) {
        self.compile_expression(value);
        self.codes.borrow_mut().push(Instruction::Dump as u8);
        self.codes.borrow_mut().push(Instruction::Drop as u8);
    }

    fn compile_expression(&mut self, value: Value) {
        if let Some(value) = value.into_typed() {
            match value {
                TypedValue::Array(array) => {
                    if let Some(symbol) = (array.get_usize(0)).to_symbol() {
                        let symbol = symbol.as_str();

                        if symbol == "fn" {
                            let mut codes = self.codes.borrow_mut();

                            codes.push(Instruction::Jump as u8);
                            let jump_target_index = codes.len();
                            codes.extend(0u32.to_le_bytes());
                            let function_index = codes.len();

                            for index in 0..array.len_usize() - 2 {
                                self.compile_statement(array.get_usize(index));
                            }

                            codes.push(Instruction::Return as u8);

                            let current_index = codes.len();

                            codes[jump_target_index..jump_target_index + size_of::<u32>()]
                                .copy_from_slice(&(current_index as u32).to_le_bytes());

                            codes.push(Instruction::Closure as u8);
                            codes.extend(function_index.to_le_bytes());
                            // TODO Initialize environment.
                            codes.extend(0u8.to_le_bytes());
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
                            self.compile_arguments(array);
                            self.codes.borrow_mut().push(instruction as u8);
                        } else {
                            self.compile_call(array);
                        }
                    } else {
                        self.compile_call(array)
                    }
                }
                TypedValue::Closure(_) => todo!(),
                TypedValue::Float64(number) => {
                    self.codes.borrow_mut().push(Instruction::Float64 as u8);
                    self.codes
                        .borrow_mut()
                        .extend(number.to_f64().to_le_bytes());
                }
                TypedValue::Symbol(symbol) => {
                    let mut codes = self.codes.borrow_mut();

                    if let Some(&index) = self.variables.get(&symbol) {
                        codes.push(Instruction::Local as u8);
                        codes.push(index as u8);
                    } else if symbol.as_str().len() >= 1 << 8 {
                        todo!();
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
    }

    fn compile_arguments(&mut self, array: Array) {
        // TODO Fix an evaluation order.
        for index in (1..array.len_usize()).rev() {
            self.compile_expression(array.get_usize(index));
        }
    }

    fn compile_call(&mut self, array: Array) {
        self.compile_arguments(array.clone());
        self.compile_expression(array.get_usize(0));
        self.codes.borrow_mut().push(Instruction::Call as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{pin_mut, FutureExt};
    use std::io;

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

        insta::assert_debug_snapshot!(codes.borrow());
    }
}
