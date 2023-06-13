mod decode;

pub use decode::{decode_instructions, DecodeError, InstructionIr};

#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::FromPrimitive)]
pub enum Instruction {
    Nil,
    Float64,
    Integer32,
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
    Close,
    Equal,
    Array,
    Drop,
    Dump,
    Jump,
    Return,
}
