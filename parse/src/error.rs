use std::{
    convert::Infallible,
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    io,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum ParseError {
    ClosedParenthesis,
    EndOfFile,
    Other(Box<dyn Error>),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::ClosedParenthesis => {
                write!(formatter, "unexpected closed parenthesis")
            }
            Self::EndOfFile => {
                write!(formatter, "unexpected end of file")
            }
            Self::Other(error) => {
                write!(formatter, "{error}")
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

impl From<Infallible> for ParseError {
    fn from(error: Infallible) -> Self {
        Self::Other(error.into())
    }
}
