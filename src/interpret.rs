mod error;
mod evaluate;

use self::{error::InterpretError, evaluate::evaluate};
use crate::expression::Expression;
use async_stream::try_stream;
use futures::{Stream, StreamExt};
use std::error::Error;

pub fn interpret_primitives<E: Error + Into<InterpretError> + 'static>(
    expressions: &mut (impl Stream<Item = Result<Expression, E>> + Unpin),
) -> impl Stream<Item = Result<Expression, InterpretError>> + '_ {
    try_stream! {
        while let Some(result) = expressions.next().await {
            yield evaluate(&result?);
        }
    }
}
