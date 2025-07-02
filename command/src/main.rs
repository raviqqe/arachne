use futures::{pin_mut, StreamExt};
use interpreter::Interpreter;
use parse::parse;
use std::{error::Error, process::exit};
use tokio::{
    fs::File,
    io::{stdin, AsyncBufReadExt, AsyncRead, BufReader},
};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error}");
        exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let matches = clap::Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .arg(clap::Arg::new("source file").required(false))
        .get_matches();

    if let Some(path) = matches.get_one::<String>("source file") {
        interpret(File::open(&path).await?).await
    } else {
        interpret(stdin()).await
    }
}

async fn interpret<T: AsyncRead>(input: T) -> Result<(), Box<dyn Error>> {
    let lines = LinesStream::new(BufReader::new(input).lines());

    pin_mut!(lines);

    let values = parse(&mut lines);

    pin_mut!(values);

    let interpreter = Interpreter::new();
    let outputs = interpreter.interpret(&mut values);

    pin_mut!(outputs);

    while let Some(result) = outputs.next().await {
        result?;
    }

    Ok(())
}
