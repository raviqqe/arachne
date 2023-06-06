use super::error::ParseError;
use async_recursion::async_recursion;
use futures::{Stream, StreamExt};
use runtime::{Array, Symbol, Value};
use std::{collections::VecDeque, error::Error, marker::Unpin};

const SPECIAL_CHARACTERS: &str = "(); \t\n";
const SYMBOL_CAPACITY: usize = 8;
const ARRAY_CAPACITY: usize = 8;
const BUFFER_CAPACITY: usize = 2 << 6;

pub struct Parser {
    buffer: VecDeque<char>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::with_capacity(BUFFER_CAPACITY),
        }
    }

    pub async fn parse_expression<E: Error + 'static>(
        &mut self,
        lines: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<Option<Value>, ParseError> {
        loop {
            if let Some(character) = self.read_character(lines).await? {
                match character {
                    '(' => return Ok(Some(self.parse_parentheses(lines).await?)),
                    ')' => return Err(ParseError::ClosedParenthesis),
                    ';' => {
                        self.parse_comment(lines).await?;
                        continue;
                    }
                    ' ' | '\t' | '\n' => continue,
                    character => return Ok(Some(self.parse_symbol(lines, character).await?)),
                }
            } else {
                return Ok(None);
            }
        }
    }

    #[async_recursion(?Send)]
    async fn parse_parentheses<E: Error + 'static>(
        &mut self,
        lines: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<Value, ParseError> {
        let mut vector = Vec::with_capacity(ARRAY_CAPACITY);

        loop {
            match self.parse_expression(lines).await {
                Err(ParseError::ClosedParenthesis) => return Ok(Array::from(vector).into()),
                Err(error) => return Err(error),
                Ok(None) => return Err(ParseError::EndOfFile),
                Ok(Some(expression)) => vector.push(expression),
            }
        }
    }

    async fn parse_symbol<E: Error + 'static>(
        &mut self,
        lines: &mut (impl Stream<Item = Result<String, E>> + Unpin),
        character: char,
    ) -> Result<Value, ParseError> {
        let mut string = String::with_capacity(SYMBOL_CAPACITY);

        string.push(character);

        loop {
            let Some(character) = self.read_character(lines).await? else { return Ok(Symbol::from(string).into()) };

            if SPECIAL_CHARACTERS.contains(character) {
                self.buffer.push_front(character);
                return Ok(Symbol::from(string).into());
            }

            string.push(character);
        }
    }

    async fn parse_comment<E: Error + 'static>(
        &mut self,
        lines: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<(), ParseError> {
        while !matches!(self.read_character(lines).await?, Some('\n') | None) {}

        Ok(())
    }

    async fn read_character<E: Error + 'static>(
        &mut self,
        lines: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<Option<char>, ParseError> {
        if self.buffer.is_empty() {
            if let Some(result) = lines.next().await {
                self.buffer.extend(
                    result
                        .map_err(|error| ParseError::Other(error.into()))?
                        .chars(),
                );
                self.buffer.push_back('\n');
            }
        }

        Ok(self.buffer.pop_front())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utility::lines_stream, ParseError};
    use futures::pin_mut;

    async fn parse(string: &str) -> Result<Option<Value>, ParseError> {
        let mut parser = Parser::new();
        let stream = lines_stream(string);

        pin_mut!(stream);

        parser.parse_expression(&mut stream).await
    }

    #[tokio::test]
    async fn parse_symbol() {
        assert_eq!(parse("foo").await.unwrap(), Some("foo".into()));
    }

    #[tokio::test]
    async fn skip_comment() {
        assert_eq!(parse(";comment\nfoo").await.unwrap(), Some("foo".into()));
    }

    #[tokio::test]
    async fn parse_array() {
        assert_eq!(parse("(foo)").await.unwrap(), Some(["foo".into()].into()));
    }
}
