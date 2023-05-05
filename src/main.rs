mod expression;
mod interpret;
mod parse;

use futures::{pin_mut, StreamExt};
use interpret::naive::interpret;
use parse::parse;
use std::error::Error;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .arg(
            clap::Arg::new("naive")
                .long("naive")
                .help("Use naive implementation")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.subcommand().is_some() {
        todo!("format subcommand")
    } else if matches.get_one("naive").copied().unwrap_or_default() {
        interpret_naive().await
    } else {
        todo!("real interpreter")
    }
}

async fn interpret_naive() -> Result<(), Box<dyn Error>> {
    let mut lines = LinesStream::new(BufReader::new(stdin()).lines());
    let expressions = parse(&mut lines);

    pin_mut!(expressions);

    let outputs = interpret(&mut expressions);

    pin_mut!(outputs);

    while let Some(result) = outputs.next().await {
        println!("{}", result?);
    }

    Ok(())
}
