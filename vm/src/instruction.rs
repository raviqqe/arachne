mod decode;

pub use decode::{decode_bytecodes, DecodeError};

#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::FromPrimitive)]
pub enum Instruction {
    Null,
    Nil,
    Float64,
    Symbol,
    Local,
    Get,
    Set,
    Length,
    Add,
    Subtract,
    Multiply,
    Divide,
    Call,
    Closure,
    Equal,
    Array,
    Drop,
    Dump,
    Jump,
    Return,
}
