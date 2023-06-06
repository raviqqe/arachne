mod error;

use async_stream::try_stream;
use error::InterpretError;
use futures::{Stream, StreamExt};
use melior::{
    dialect::DialectRegistry,
    ir::{Location, Module},
    utility::register_all_dialects,
    Context, ExecutionEngine,
};
use runtime::{Value, NIL};
use std::error::Error;

pub fn interpret<E: Error + 'static>(
    expressions: &mut (impl Stream<Item = Result<Value, E>> + Unpin),
) -> impl Stream<Item = Result<Value, InterpretError>> + '_ {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.get_or_load_dialect("func");

    let location = Location::unknown(&context);
    let module = Module::new(location);
    let _engine = ExecutionEngine::new(&module, 2, &[], false);

    try_stream! {
        while let Some(_result) = expressions.next().await {
            // TODO
            yield NIL;
        }
    }
}
