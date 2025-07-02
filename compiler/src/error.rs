use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    num::TryFromIntError,
};

#[derive(Debug)]
pub enum CompileError {
    Closure,
    Other(Box<dyn Error>),
    Syntax(String),
    SymbolLength(String),
    VariableNotDefined(String),
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Closure => {
                write!(formatter, "closure cannot be compiled")
            }
            Self::Other(error) => {
                write!(formatter, "{error}")
            }
            Self::Syntax(literal) => {
                write!(formatter, "invalid syntax: {literal}")
            }
            Self::SymbolLength(symbol) => {
                write!(formatter, "symbol too long: {symbol}")
            }
            Self::VariableNotDefined(symbol) => {
                write!(formatter, "variable not found: {symbol}")
            }
        }
    }
}

impl From<TryFromIntError> for CompileError {
    fn from(error: TryFromIntError) -> Self {
        Self::Other(error.into())
    }
}
