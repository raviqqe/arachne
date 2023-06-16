use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum InstructionIr {
    Drop,
    Peek(u8),
    Nil,
    Float64(f64),
    Integer32(i32),
    Symbol {
        len: u8,
        string: String,
    },
    Close {
        pointer: u32,
        arity: u8,
        environment_size: u8,
        environment: Vec<u8>,
    },
    Environment(u8),
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Get,
    Set,
    Length,
    Call {
        arity: u8,
    },
    Jump {
        pointer: i16,
    },
    Branch {
        pointer: i16,
    },
    Return,
    Dump,
}

impl Display for InstructionIr {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Drop => write!(formatter, "drop"),
            Self::Peek(index) => write!(formatter, "peek {}", index),
            Self::Nil => write!(formatter, "nil"),
            Self::Float64(number) => write!(formatter, "float64 {}", number),
            Self::Integer32(number) => write!(formatter, "integer32 {}", number),
            Self::Symbol { len, string } => write!(formatter, "symbol {} {:?}", len, string),
            Self::Close {
                pointer,
                arity,
                environment_size,
                environment,
            } => write!(
                formatter,
                "close {:x} {} {} {:?}",
                pointer, arity, environment_size, environment
            ),
            Self::Environment(index) => write!(formatter, "environment {}", index),
            Self::Equal => write!(formatter, "equal"),
            Self::Add => write!(formatter, "add"),
            Self::Subtract => write!(formatter, "subtract"),
            Self::Multiply => write!(formatter, "multiply"),
            Self::Divide => write!(formatter, "divide"),
            Self::Get => write!(formatter, "get"),
            Self::Set => write!(formatter, "set"),
            Self::Length => write!(formatter, "length"),
            Self::Call { arity } => write!(formatter, "call {}", arity),
            Self::Jump { pointer } => write!(formatter, "jump {:x}", pointer),
            Self::Branch { pointer } => write!(formatter, "branch {:x}", pointer),
            Self::Return => write!(formatter, "return"),
            Self::Dump => write!(formatter, "dump"),
        }
    }
}
