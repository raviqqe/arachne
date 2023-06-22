use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum InstructionIr {
    Add,
    And,
    Branch {
        pointer: i16,
    },
    Call {
        arity: u8,
    },
    Close {
        pointer: u64,
        arity: u64,
        environment_size: u64,
    },
    Divide,
    Drop,
    Dump,
    Environment(u8),
    Equal,
    Float64(f64),
    Get,
    Integer32(i32),
    Jump {
        pointer: i16,
    },
    Length,
    LessThan,
    Multiply,
    Nil,
    Not,
    Or,
    Peek(u8),
    Return,
    Set,
    Subtract,
    Symbol {
        len: u8,
        string: String,
    },
    TailCall {
        arity: u64,
    },
}

impl Display for InstructionIr {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Add => write!(formatter, "add"),
            Self::And => write!(formatter, "and"),
            Self::Branch { pointer } => write!(formatter, "branch {:x}", pointer),
            Self::Call { arity } => write!(formatter, "call {}", arity),
            Self::Close {
                pointer,
                arity,
                environment_size,
            } => write!(
                formatter,
                "close {:x} {} {}",
                pointer, arity, environment_size
            ),
            Self::Divide => write!(formatter, "divide"),
            Self::Drop => write!(formatter, "drop"),
            Self::Dump => write!(formatter, "dump"),
            Self::Environment(index) => write!(formatter, "environment {}", index),
            Self::Equal => write!(formatter, "equal"),
            Self::Float64(number) => write!(formatter, "float64 {}", number),
            Self::Get => write!(formatter, "get"),
            Self::Integer32(number) => write!(formatter, "integer32 {}", number),
            Self::Jump { pointer } => write!(formatter, "jump {:x}", pointer),
            Self::Length => write!(formatter, "length"),
            Self::LessThan => write!(formatter, "less_than"),
            Self::Multiply => write!(formatter, "multiply"),
            Self::Nil => write!(formatter, "nil"),
            Self::Not => write!(formatter, "not"),
            Self::Or => write!(formatter, "or"),
            Self::Peek(index) => write!(formatter, "peek {}", index),
            Self::Return => write!(formatter, "return"),
            Self::Set => write!(formatter, "set"),
            Self::Subtract => write!(formatter, "subtract"),
            Self::Symbol { len, string } => write!(formatter, "symbol {} {:?}", len, string),
            Self::TailCall { arity } => write!(formatter, "tail_call {}", arity),
        }
    }
}
