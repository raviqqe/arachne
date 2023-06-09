use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CompileError {
    Closure,
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Closure => {
                write!(formatter, "closure cannnot be compiled")
            }
        }
    }
}
