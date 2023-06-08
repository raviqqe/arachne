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
                compile_statement(value?, &mut self.codes.borrow_mut());
                yield ();
            }
        }
    }
}

fn compile_statement(value: Value, codes: &mut Vec<u8>) {
    let Some(array) = value.into_array() else { return };

    if let Some(symbol) = array.get_usize(0).to_symbol() {
        match symbol.as_str() {
            // TODO Generate let instruction.
            "let" => todo!(),
            _ => compile_expression(array.into(), codes),
        }
    } else {
        compile_expression(array.into(), codes);
    }
}

fn compile_expression(value: Value, codes: &mut Vec<u8>) {
    if let Some(value) = value.into_typed() {
        match value {
            TypedValue::Array(array) => match Symbol::try_from(array.get_usize(0)) {
                Ok(symbol) => match symbol.as_str() {
                    "array" => todo!(),
                    "eq" => todo!(),
                    "get" => {
                        compile_arguments(array, 2, codes);
                        codes.push(Instruction::Get as u8);
                    }
                    "set" => {
                        compile_arguments(array, 3, codes);
                        codes.push(Instruction::Set as u8);
                    }
                    "len" => {
                        compile_arguments(array, 1, codes);
                        codes.push(Instruction::Length as u8);
                    }
                    _ => compile_call(array, codes),
                },
                Err(value) => compile_call(array, codes),
            },
            TypedValue::Closure(closure) => compile_constant(closure, codes),
            TypedValue::Float64(number) => compile_constant(number, codes),
            TypedValue::Symbol(symbol) => compile_variable(symbol, codes),
        }
    } else {
        codes.push(Instruction::Constant as u8);
        codes.extend(NIL.into_raw().to_le_bytes());
    }
}

fn compile_arguments(array: Array, arity: usize, codes: &mut Vec<u8>) {
    foo
}

fn compile_constant<T: Into<Value>>(value: T, codes: &mut Vec<u8>) {
    codes.push(Instruction::Constant as u8);
    codes.extend(value.into().into_raw().to_le_bytes());
}

fn compile_variable(_symbol: Symbol, codes: &mut Vec<u8>) {
    codes.push(Instruction::Local as u8);
    todo!("Resolve a symbol.")
}

fn compile_call(_array: Array, codes: &mut Vec<u8>) {
    codes.push(Instruction::Call as u8);
    todo!()
}
