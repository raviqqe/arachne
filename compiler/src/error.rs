use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum CompileError {
    Closure,
    Other(Box<dyn Error>),
    SymbolLength(String),
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Closure => {
                write!(formatter, "closure cannnot be compiled")
            }
            Self::Other(error) => {
                write!(formatter, "{}", error)
            }
            Self::SymbolLength(symbol) => {
                write!(formatter, "symbol too long: {}", symbol)
            }
        }
    }
}
