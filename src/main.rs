mod expression;
mod parse;

use futures::{pin_mut, StreamExt};
use parse::parse;
use std::error::Error;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = LinesStream::new(BufReader::new(stdin()).lines());
    let expressions = parse(&mut lines);

    pin_mut!(expressions);

    while let Some(result) = expressions.next().await {
        println!("{:?}", result?);
    }

    Ok(())
}
