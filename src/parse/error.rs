use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ParseError {
    ClosedParenthesis,
    Other(Box<dyn Error>),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ClosedParenthesis => {
                write!(formatter, "stray closed parenthesis")
            }
            Self::Other(error) => {
                write!(formatter, "{}", error)
            }
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        Self::Other(error.into())
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(error: FromUtf8Error) -> Self {
        Self::Other(error.into())
    }
}
