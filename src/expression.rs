#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Symbol(String),
    Array(Vec<Expression>),
}
