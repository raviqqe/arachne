mod expression;
mod parse;

use parse::parse_expression;
use std::error::Error;
use tokio::io::stdin;

#[derive(Debug, Default)]
enum Expression {
    #[default]
    None,
    Symbol(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();

    loop {
        let expression = parse_expression(&mut stdin)?;
    }

    Ok(())
}
