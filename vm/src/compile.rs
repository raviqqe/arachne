use crate::instruction::Instruction;
use runtime::{TypedValue, Value};

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
    match value.into_enum() {
        TypedValue::Nil => {
            codes.push(Instruction::Constant as u8);
            codes.extend(value.into_raw().to_le_bytes());
        }
        TypedValue::Array(array) => match array.get_usize(0).into_symbol() {
            Ok(symbol) => match symbol.as_str() {
                "array" => todo!(),
                "eq" => todo!(),
                "get" => todo!(),
                "set" => todo!(),
                "len" => todo!(),
                _ => compile_call(value, codes),
            },
            Err(value) => {
                compile_expression(todo!(), codes);
                compile_call(value, codes);
            }
        },
        TypedValue::Nil => {
            codes.push(Instruction::Constant as u8);
            codes.extend(value.into_raw().to_le_bytes());
        }
    }
}

fn compile_call(value: Value, codes: &mut Vec<u8>) {
    todo!()
}
