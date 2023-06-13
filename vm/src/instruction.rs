mod decode;

pub use decode::{decode_instructions, DecodeError, InstructionIr};

#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::FromPrimitive)]
pub enum Instruction {
    Nil,
    Float64,
    Integer32,
    Symbol,
    Peek,
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
    Drop,
    Dump,
    Jump,
    Return,
}
