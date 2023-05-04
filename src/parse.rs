mod error;
mod parser;

use self::error::ParseError;
use crate::expression::Expression;
use async_stream::try_stream;
use futures::Stream;
use parser::Parser;
use std::error::Error;

pub fn parse<E: Error + 'static>(
    stream: &mut (impl Stream<Item = Result<String, E>> + Unpin),
) -> impl Stream<Item = Result<Expression, ParseError>> + '_ {
    try_stream! {
        let mut parser = Parser::new();

        loop {
            let expression = parser.parse_expression(stream).await?;

            if let Some(expression) = expression {
                yield expression;
            } else {
                break;
            }
        }
    }
}
