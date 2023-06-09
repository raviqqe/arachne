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


// Only for debugging.
#[derive(Clone, Debug)]
enum InstructionNode {
    Null,
    Nil,
    Float64(f64),
    Symbol {
        len: u8,
        string: String,
    },
    Local,
    Get,
    Set,
    Length,
    Add,
    Subtract,
    Multiply,
    Divide,
    Call,
    Closure {
        pointer: u32,
        environment_size: u8,
        environment: Vec<u8>,
    },
    Equal,
    Array,
    Drop,
    Dump,
    Jump,
    Return,
}

pub fn decode(&[u8]) -> Vec<InstructionNode> { 
let mut instructions = Vec::new();

    instructions
}
