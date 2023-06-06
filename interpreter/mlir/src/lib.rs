mod error;

use ast::Expression;
use async_stream::try_stream;
use error::InterpretError;
use futures::{Stream, StreamExt};
use melior::{
    dialect::DialectRegistry,
    ir::{Location, Module},
    utility::register_all_dialects,
    Context, ExecutionEngine,
};
use std::error::Error;

pub fn interpret<E: Error + Into<InterpretError> + 'static>(
    expressions: &mut (impl Stream<Item = Result<Expression, E>> + Unpin),
) -> impl Stream<Item = Result<Expression, InterpretError>> + '_ {
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
            yield Expression::Array(vec![]);
        }
    }
}
