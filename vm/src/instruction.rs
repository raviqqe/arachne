mod format;

pub use format::{format_instructions, FormatError};

#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::FromPrimitive)]
pub enum Instruction {
    // Stack operation
    Drop,
    Peek,

    // Computation
    Add,
    Subtract,
    Multiply,
    Divide,
    Get,
    Set,
    Length,
    Equal,

    // Constant
    Nil,
    Float64,
    Integer32,
    Symbol,

    // Closure
    Close,
    Environment,

    // Control
    Call,
    Return,
    Jump,
    Branch,

    // Debug
    Dump,
}
