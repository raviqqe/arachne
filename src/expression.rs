use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Symbol(String),
    Array(Vec<Expression>),
}

impl From<&str> for Expression {
    fn from(string: &str) -> Self {
        Expression::Symbol(string.into())
    }
}

impl From<String> for Expression {
    fn from(string: String) -> Self {
        Expression::Symbol(string)
    }
}

impl From<Vec<Expression>> for Expression {
    fn from(array: Vec<Expression>) -> Self {
        Expression::Array(array)
    }
}

impl Display for Expression {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Symbol(symbol) => write!(formatter, "{}", symbol),
            Expression::Array(array) => {
                write!(formatter, "(")?;

                for expression in array {
                    write!(formatter, "{}", expression)?;
                }

                write!(formatter, ")")?;

                Ok(())
            }
        }
    }
}
