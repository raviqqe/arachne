use parse::ParseError;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum InterpretError {
    Parse(ParseError),
}

impl Display for InterpretError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(formatter, "{}", error),
        }
    }
}

impl Error for InterpretError {}

impl From<ParseError> for InterpretError {
    fn from(error: ParseError) -> Self {
        Self::Parse(error)
    }
}
