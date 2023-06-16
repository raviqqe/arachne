use futures::{pin_mut, StreamExt};
use interpreter::Interpreter;
use parse::parse;
use std::{error::Error, process::exit};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

const BYTECODE_CAPACITY: usize = 1 << 10;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{}", error);
        exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    clap::Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .get_matches();

    let mut lines = LinesStream::new(BufReader::new(stdin()).lines());
    let values = parse(&mut lines);

    pin_mut!(values);

    let interpreter = Interpreter::new(BYTECODE_CAPACITY);
    let outputs = interpreter.interpret(&mut values);

    pin_mut!(outputs);

    while let Some(result) = outputs.next().await {
        result?;
    }

    Ok(())
}
