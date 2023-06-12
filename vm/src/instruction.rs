mod decode;

pub use decode::{decode_instructions, DecodeError};

#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::FromPrimitive)]
pub enum Instruction {
    Nil,
    Float64,
    Symbol,
    Global,
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
