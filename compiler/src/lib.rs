use async_stream::try_stream;
use futures::{Stream, StreamExt};
use runtime::{Array, Symbol, TypedValue, Value, NIL};
use std::{cell::RefCell, error::Error};
use vm::Instruction;

pub struct Compiler<'a> {
    codes: &'a RefCell<Vec<u8>>,
}

impl<'a> Compiler<'a> {
    pub fn new(codes: &'a RefCell<Vec<u8>>) -> Self {
        Self { codes }
    }

    pub fn compile<'b, E: Error + 'static>(
        &'b self,
        values: &'b mut (impl Stream<Item = Result<Value, E>> + Unpin),
    ) -> impl Stream<Item = Result<(), E>> + 'b {
        try_stream! {
            while let Some(value) = values.next().await {
                self.compile_statement(value?);
                yield ();
            }
        }
    }

    fn compile_statement(&self, value: Value) {
        let Some(array) = value.into_array() else { return };

        if let Some(symbol) = array.get_usize(0).to_symbol() {
            match symbol.as_str() {
                // TODO Generate let instruction.
                "let" => todo!(),
                _ => self.compile_expression(array.into()),
            }
        } else {
            self.compile_expression(array.into());
        }
    }

    fn compile_expression(&self, value: Value) {
        if let Some(value) = value.into_typed() {
            match value {
                TypedValue::Array(array) => match Symbol::try_from(array.get_usize(0)) {
                    Ok(symbol) => match symbol.as_str() {
                        "array" => todo!(),
                        "eq" => todo!(),
                        "get" => {
                            self.compile_arguments(array);
                            self.codes.borrow_mut().push(Instruction::Get as u8);
                        }
                        "set" => {
                            self.compile_arguments(array);
                            self.codes.borrow_mut().push(Instruction::Set as u8);
                        }
                        "len" => {
                            self.compile_arguments(array);
                            self.codes.borrow_mut().push(Instruction::Length as u8);
                        }
                        _ => self.compile_call(array),
                    },
                    Err(value) => self.compile_call(array),
                },
                TypedValue::Closure(closure) => self.compile_constant(closure),
                TypedValue::Float64(number) => self.compile_constant(number),
                TypedValue::Symbol(symbol) => self.compile_variable(symbol),
            }
        } else {
            self.codes.borrow_mut().push(Instruction::Constant as u8);
            self.codes.borrow_mut().extend(NIL.into_raw().to_le_bytes());
        }
    }

    fn compile_arguments(&self, array: Array) {
        for index in (1..array.len_usize()).rev() {
            self.compile_expression(array.get_usize(index));
        }
    }

    fn compile_constant<T: Into<Value>>(&self, value: T) {
        self.codes.borrow_mut().push(Instruction::Constant as u8);
        self.codes
            .borrow_mut()
            .extend(value.into().into_raw().to_le_bytes());
    }

    fn compile_variable(&self, _symbol: Symbol) {
        self.codes.borrow_mut().push(Instruction::Local as u8);
        todo!("Resolve a symbol.")
    }

    fn compile_call(&self, array: Array) {
        self.compile_arguments(array.clone());
        self.compile_expression(array.get_usize(0));
        self.codes.borrow_mut().push(Instruction::Call as u8);
    }
}
