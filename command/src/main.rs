mod expression;
mod interpret;
mod parse;
mod runtime;

use futures::{pin_mut, StreamExt};
use interpret::{mlir, naive};
use parse::parse;
use std::error::Error;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

macro_rules! interpret_fn {
    ($name: ident, $stream_fn: expr) => {
        async fn $name() -> Result<(), Box<dyn Error>> {
            let mut lines = LinesStream::new(BufReader::new(stdin()).lines());
            let expressions = parse(&mut lines);

            pin_mut!(expressions);

            let outputs = $stream_fn(&mut expressions);

            pin_mut!(outputs);

            while let Some(result) = outputs.next().await {
                println!("{}", result?);
            }

            Ok(())
        }
    };
}

interpret_fn!(interpret_naive, naive::interpret);
interpret_fn!(interpret_mlir, mlir::interpret);

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
        interpret_mlir().await
    }
}
