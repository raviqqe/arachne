#[derive(Debug)]
pub enum Expression {
    Symbol(String),
    Array(Vec<Expression>),
}
