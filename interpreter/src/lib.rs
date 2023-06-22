mod error;

use async_stream::try_stream;
use compiler::Compiler;
use error::InterpretError;
use futures::{pin_mut, Stream, StreamExt};
use runtime::Value;
use runtime::Vm;
use std::{cell::RefCell, error::Error};

#[derive(Debug, Default)]
pub struct Interpreter {
    codes: RefCell<Vec<u8>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            codes: Default::default(),
        }
    }

    pub fn interpret<'a, E: Error + 'static>(
        &'a self,
        values: &'a mut (impl Stream<Item = Result<Value, E>> + Unpin),
    ) -> impl Stream<Item = Result<(), InterpretError>> + 'a {
        try_stream! {
            let mut compiler = Compiler::new(&self.codes);
            let mut vm = Vm::new();
            let results = compiler.compile(values);

            pin_mut!(results);

            while let Some(result) = results.next().await {
                result.map_err(|error| InterpretError::Other(error.into()))?;
                vm.run(&self.codes.borrow());
                yield ();
            }
        }
    }
}
