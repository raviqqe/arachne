mod error;

use compiler::compile;
use error::InterpretError;
use futures::{Stream, StreamExt};
use runtime::Value;
use std::error::Error;
use vm::Vm;

const BYTE_CODE_CAPACITY: usize = 1 << 10;
const VM_STACK_SIZE: usize = 1 << 10;

pub async fn interpret<E: Error + 'static>(
    values: &mut (impl Stream<Item = Result<Value, E>> + Unpin),
) -> Result<(), InterpretError> {
    let mut codes = Vec::with_capacity(BYTE_CODE_CAPACITY);
    let values = values.collect::<Vec<_>>().await;

    compile(
        values
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| InterpretError::Other(error.into()))?,
        &mut codes,
    );

    let mut vm = Vm::new(VM_STACK_SIZE);

    vm.run(&codes);

    Ok(())
}
