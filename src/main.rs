mod expression;
mod parse;

use parse::parse_expression;
use std::error::Error;
use tokio::io::stdin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = stdin();

    loop {
        let expression = parse_expression(&mut stdin).await?;

        if let Some(_expression) = expression {
            todo!();
        } else {
            break;
        }
    }

    Ok(())
}
