mod format;

pub use format::{format_instructions, FormatError};

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
    LessThan,
    Not,
    And,
    Or,

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
    TailCall,
    Return,
    Jump,
    Branch,

    // Debug
    Dump,
}

impl Instruction {
    pub const DROP: u64 = Self::Drop as _;
    pub const PEEK: u64 = Self::Peek as _;
    pub const ADD: u64 = Self::Add as _;
    pub const SUBTRACT: u64 = Self::Subtract as _;
    pub const MULTIPLY: u64 = Self::Multiply as _;
    pub const DIVIDE: u64 = Self::Divide as _;
    pub const GET: u64 = Self::Get as _;
    pub const SET: u64 = Self::Set as _;
    pub const LENGTH: u64 = Self::Length as _;
    pub const EQUAL: u64 = Self::Equal as _;
    pub const LESS_THAN: u64 = Self::LessThan as _;
    pub const NOT: u64 = Self::Not as _;
    pub const AND: u64 = Self::And as _;
    pub const OR: u64 = Self::Or as _;
    pub const NIL: u64 = Self::Nil as _;
    pub const FLOAT64: u64 = Self::Float64 as _;
    pub const INTEGER32: u64 = Self::Integer32 as _;
    pub const SYMBOL: u64 = Self::Symbol as _;
    pub const CLOSE: u64 = Self::Close as _;
    pub const ENVIRONMENT: u64 = Self::Environment as _;
    pub const CALL: u64 = Self::Call as _;
    pub const TAIL_CALL: u64 = Self::TailCall as _;
    pub const RETURN: u64 = Self::Return as _;
    pub const JUMP: u64 = Self::Jump as _;
    pub const BRANCH: u64 = Self::Branch as _;
    pub const DUMP: u64 = Self::Dump as _;
}
