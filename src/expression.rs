#[derive(Debug, Default)]
pub enum Expression {
    #[default]
    None,
    Symbol(String),
    Parentheses(Vec<Expression>),
}
