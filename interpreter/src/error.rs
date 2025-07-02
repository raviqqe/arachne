use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum InterpretError {
    Other(Box<dyn Error>),
}

impl Display for InterpretError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Other(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for InterpretError {}
