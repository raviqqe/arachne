use crate::instruction::Instruction;
use runtime::{Array, Symbol, TypedValue, Value, NIL};

pub fn compile(values: impl IntoIterator<Item = Value>, codes: &mut Vec<u8>) {
    for value in values {
        compile_statement(value, codes);
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
                        codes.push(Instruction::Get as u8);
                        todo!();
                    }
                    "set" => {
                        codes.push(Instruction::Set as u8);
                        todo!();
                    }
                    "len" => {
                        codes.push(Instruction::Length as u8);
                        todo!();
                    }
                    _ => {
                        compile_variable(symbol, codes);
                        compile_call(array.into(), codes);
                    }
                },
                Err(value) => {
                    compile_expression(value, codes);
                    compile_call(array, codes);
                }
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

fn compile_constant<T: Into<Value>>(value: T, codes: &mut Vec<u8>) {
    codes.push(Instruction::Constant as u8);
    codes.extend(value.into().into_raw().to_le_bytes());
}

fn compile_variable(symbol: Symbol, codes: &mut Vec<u8>) {
    todo!("Resolve a symbol.")
}

fn compile_call(array: Array, codes: &mut Vec<u8>) {
    codes.push(Instruction::Call as u8);
    todo!()
}
