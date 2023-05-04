mod expression;
mod parse;

use parse::parse_expression;
use std::error::Error;

#[derive(Debug, Default)]
enum Expression {
    #[default]
    None,
    Symbol(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        parse_expression()?
    }
}
