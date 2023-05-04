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

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::{expression::Expression, parse::error::ParseError};
    use async_stream::stream;
    use futures::{pin_mut, StreamExt};
    use std::io;

    async fn parse_string(string: &str) -> Result<Vec<Expression>, ParseError> {
        // TODO Can we covnert &str into Stream directly?
        let stream = stream! {
            for line in string.lines() {
                yield Ok::<_, io::Error>(line.trim().to_owned());
            }
        };

        pin_mut!(stream);

        parse(&mut stream)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect()
    }

    #[tokio::test]
    async fn parse_symbol() {
        assert_eq!(
            parse_string("foo").await.unwrap(),
            vec![Expression::Symbol("foo".into())]
        );
    }

    #[tokio::test]
    async fn parse_symbols_separated_by_space() {
        assert_eq!(
            parse_string("foo bar").await.unwrap(),
            vec![
                Expression::Symbol("foo".into()),
                Expression::Symbol("bar".into())
            ]
        );
    }

    #[tokio::test]
    async fn parse_symbols_separated_by_newline() {
        assert_eq!(
            parse_string("foo\nbar").await.unwrap(),
            vec![
                Expression::Symbol("foo".into()),
                Expression::Symbol("bar".into())
            ]
        );
    }

    #[tokio::test]
    async fn parse_symbols_separated_by_tab() {
        assert_eq!(
            parse_string("foo\tbar").await.unwrap(),
            vec![
                Expression::Symbol("foo".into()),
                Expression::Symbol("bar".into())
            ]
        );
    }

    #[tokio::test]
    async fn skip_comment() {
        assert_eq!(
            parse_string(";comment\nfoo").await.unwrap(),
            vec![Expression::Symbol("foo".into())]
        );
    }

    #[tokio::test]
    async fn parse_array() {
        assert_eq!(
            parse_string("(foo)").await.unwrap(),
            vec![Expression::Array(vec![Expression::Symbol("foo".into())])]
        );
    }
}
