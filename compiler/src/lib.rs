use async_stream::try_stream;
use futures::{Stream, StreamExt};
use runtime::{Array, Symbol, TypedValue, Value};
use std::{cell::RefCell, collections::HashMap, error::Error};
use vm::Instruction;

const VARIABLE_CAPACITY: usize = 1 << 10;

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
        match Array::try_from(value) {
            Ok(array) => {
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
            Err(value) => self.compile_expression(value),
        }

        self.codes.borrow_mut().push(Instruction::Dump as u8);
    }

    fn compile_expression(&self, value: Value) {
        if let Some(value) = value.into_typed() {
            match value {
                TypedValue::Array(array) => {
                    if let Some(symbol) = (array.get_usize(0)).to_symbol() {
                        if let Some(instruction) = match symbol.as_str() {
                            "array" => Some(Instruction::Array),
                            "eq" => Some(Instruction::Equal),
                            "get" => Some(Instruction::Get),
                            "set" => Some(Instruction::Set),
                            "len" => Some(Instruction::Length),
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
                TypedValue::Closure(closure) => self.compile_constant(closure),
                TypedValue::Float64(number) => self.compile_constant(number),
                TypedValue::Symbol(symbol) => self.compile_variable(symbol),
            }
        } else {
            self.codes.borrow_mut().push(Instruction::Nil as u8);
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
