mod expression;
mod parse;

use parse::Parser;
use std::error::Error;
use tokio::io::stdin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = stdin();
    let mut parser = Parser::new();

    loop {
        let expression = parser.parse_expression(&mut stdin).await?;

        if let Some(expression) = expression {
            println!("{:?}", expression)
        } else {
            break;
        }
    }

    Ok(())
}
