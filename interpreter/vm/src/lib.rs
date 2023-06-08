mod error;

use async_stream::try_stream;
use compiler::Compiler;
use error::InterpretError;
use futures::{pin_mut, Stream, StreamExt};
use runtime::{Value, NIL};
use std::{cell::RefCell, error::Error};
use vm::Vm;

const VM_STACK_SIZE: usize = 1 << 10;

pub struct Interpreter {
    codes: RefCell<Vec<u8>>,
}

impl Interpreter {
    pub fn new(byte_code_capacity: usize) -> Self {
        Self {
            codes: Vec::with_capacity(byte_code_capacity).into(),
        }
    }

    pub fn interpret<'a, E: Error + 'static>(
        &'a self,
        values: &'a mut (impl Stream<Item = Result<Value, E>> + Unpin),
    ) -> impl Stream<Item = Result<Value, InterpretError>> + 'a {
        try_stream! {
            let mut compiler = Compiler::new(&self.codes);
            let mut vm = Vm::new(VM_STACK_SIZE);
            let results = compiler.compile(values);

            pin_mut!(results);

            while let Some(result) = results.next().await {
                result.map_err(|error| InterpretError::Other(error.into()))?;
                vm.run(&self.codes.borrow());
                yield NIL;
            }
        }
    }
}
