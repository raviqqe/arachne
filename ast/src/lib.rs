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

                for (index, expression) in array.iter().enumerate() {
                    if index != 0 {
                        write!(formatter, " ")?;
                    }

                    write!(formatter, "{}", expression)?;
                }

                write!(formatter, ")")?;

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn format_symbol() {
        assert_eq!(Expression::from("foo").to_string(), "foo");
    }

    #[test]
    fn format_array() {
        assert_eq!(
            Expression::from(vec!["foo".into(), "bar".into(), "baz".into()]).to_string(),
            "(foo bar baz)"
        );
    }
}
