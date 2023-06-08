use futures::{pin_mut, StreamExt};
use parse::parse;
use std::error::Error;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;
use vm_interpreter::Interpreter;

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

const BYTE_CODE_CAPACITY: usize = 1 << 10;

interpret_fn!(interpret_naive, naive_interpreter::interpret);
interpret_fn!(interpret_mlir, mlir_interpreter::interpret);

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
        .arg(
            clap::Arg::new("mlir")
                .long("mlir")
                .help("Use mlir implementation")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.subcommand().is_some() {
        todo!("format subcommand")
    } else if matches.get_one("naive").copied().unwrap_or_default() {
        interpret_naive().await
    } else if matches.get_one("mlir").copied().unwrap_or_default() {
        interpret_mlir().await
    } else {
        let mut lines = LinesStream::new(BufReader::new(stdin()).lines());
        let values = parse(&mut lines);

        pin_mut!(values);

        let interpreter = Interpreter::new(BYTE_CODE_CAPACITY);
        let outputs = interpreter.interpret(&mut values);

        pin_mut!(outputs);

        while let Some(result) = outputs.next().await {
            result?;
        }

        Ok(())
    }
}
