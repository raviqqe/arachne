mod format;

pub use format::{format_instructions, FormatError};

#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::FromPrimitive)]
pub enum Instruction {
    Add,
    And,
    Branch,
    Call,
    Close,
    Divide,
    Drop,
    Dump,
    Environment,
    Equal,
    Float64,
    Get,
    Integer32,
    Jump,
    Length,
    LessThan,
    Multiply,
    Nil,
    Not,
    Or,
    Peek,
    Return,
    Set,
    Subtract,
    Symbol,
    TailCall,
}
