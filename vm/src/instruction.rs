mod format;

pub use format::{format_instructions, FormatError};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
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
    pub const DROP: u8 = Self::Drop as _;
    pub const PEEK: u8 = Self::Peek as _;
    pub const ADD: u8 = Self::Add as _;
    pub const SUBTRACT: u8 = Self::Subtract as _;
    pub const MULTIPLY: u8 = Self::Multiply as _;
    pub const DIVIDE: u8 = Self::Divide as _;
    pub const GET: u8 = Self::Get as _;
    pub const SET: u8 = Self::Set as _;
    pub const LENGTH: u8 = Self::Length as _;
    pub const EQUAL: u8 = Self::Equal as _;
    pub const LESS_THAN: u8 = Self::LessThan as _;
    pub const NOT: u8 = Self::Not as _;
    pub const AND: u8 = Self::And as _;
    pub const OR: u8 = Self::Or as _;
    pub const NIL: u8 = Self::Nil as _;
    pub const FLOAT64: u8 = Self::Float64 as _;
    pub const INTEGER32: u8 = Self::Integer32 as _;
    pub const SYMBOL: u8 = Self::Symbol as _;
    pub const CLOSE: u8 = Self::Close as _;
    pub const ENVIRONMENT: u8 = Self::Environment as _;
    pub const CALL: u8 = Self::Call as _;
    pub const TAIL_CALL: u8 = Self::TailCall as _;
    pub const RETURN: u8 = Self::Return as _;
    pub const JUMP: u8 = Self::Jump as _;
    pub const BRANCH: u8 = Self::Branch as _;
    pub const DUMP: u8 = Self::Dump as _;
}
