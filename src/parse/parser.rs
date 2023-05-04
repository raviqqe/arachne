use super::error::ParseError;
use crate::expression::Expression;
use async_recursion::async_recursion;
use futures::Stream;
use futures::StreamExt;
use std::error::Error;
use std::marker::Unpin;

const SPECIAL_CHARACTERS: &str = "(); \t\n";
const SYMBOL_CAPACITY: usize = 8;
const ARRAY_CAPACITY: usize = 8;
const BUFFER_CAPACITY: usize = 8;

pub struct Parser {
    buffer: String,
    offset: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(BUFFER_CAPACITY),
            offset: 0,
        }
    }

    pub async fn parse_expression<E: Error + 'static>(
        &mut self,
        reader: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<Option<Expression>, ParseError> {
        loop {
            if let Some(character) = self.read_character(reader).await? {
                match character {
                    '(' => return Ok(Some(self.parse_parentheses(reader).await?)),
                    ')' => return Err(ParseError::ClosedParenthesis),
                    ';' => {
                        self.parse_comment(reader).await?;
                        continue;
                    }
                    ' ' | '\t' | '\n' => continue,
                    character => return Ok(Some(self.parse_symbol(reader, character).await?)),
                }
            } else {
                return Ok(None);
            }
        }
    }

    #[async_recursion(?Send)]
    async fn parse_parentheses<E: Error + 'static>(
        &mut self,
        reader: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<Expression, ParseError> {
        let mut vector = Vec::with_capacity(ARRAY_CAPACITY);

        loop {
            match self.parse_expression(reader).await {
                Err(ParseError::ClosedParenthesis) => return Ok(Expression::Array(vector)),
                Err(error) => return Err(error),
                Ok(None) => return Err(ParseError::EndOfFile),
                Ok(Some(expression)) => vector.push(expression),
            }
        }
    }

    async fn parse_symbol<E: Error + 'static>(
        &mut self,
        reader: &mut (impl Stream<Item = Result<String, E>> + Unpin),
        character: char,
    ) -> Result<Expression, ParseError> {
        let mut string = String::with_capacity(SYMBOL_CAPACITY);

        string.push(character);

        loop {
            let character = self.read_character(reader).await?;

            if character
                .map(|character| SPECIAL_CHARACTERS.contains(character))
                .unwrap_or(true)
            {
                self.buffer.extend(character);
                return Ok(Expression::Symbol(string));
            }

            string.extend(character);
        }
    }

    async fn parse_comment<E: Error + 'static>(
        &mut self,
        reader: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<(), ParseError> {
        while !matches!(self.read_character(reader).await?, Some('\n') | None) {}

        Ok(())
    }

    async fn read_character<E: Error + 'static>(
        &mut self,
        reader: &mut (impl Stream<Item = Result<String, E>> + Unpin),
    ) -> Result<Option<char>, ParseError> {
        if self.buffer.len() == self.offset {
            match reader.next().await {
                None => return Ok(None),
                Some(Ok(string)) => {
                    self.buffer.extend(string.chars());
                    self.buffer.push('\n');
                }
                Some(Err(error)) => return Err(ParseError::Other(error.into())),
            }
        }

        self.offset += 1;

        // TODO This is O(n).
        Ok(self.buffer.chars().nth(self.offset - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{expression::Expression, parse::error::ParseError};
    use async_stream::stream;
    use futures::pin_mut;

    async fn parse(string: &str) -> Result<Option<Expression>, ParseError> {
        let mut parser = Parser::new();
        // TODO Can we covnert &str into Stream directly?
        let stream = stream! {
            for line in string.lines() {
                yield Ok(line.trim().to_owned());
            }
        };

        pin_mut!(stream);

        parser.parse_expression::<ParseError>(&mut stream).await
    }

    #[tokio::test]
    async fn parse_symbol() {
        assert_eq!(
            parse("foo").await.unwrap(),
            Some(Expression::Symbol("foo".into()))
        );
    }

    #[tokio::test]
    async fn skip_comment() {
        assert_eq!(
            parse(";comment\nfoo").await.unwrap(),
            Some(Expression::Symbol("foo".into()))
        );
    }

    #[tokio::test]
    async fn parse_array() {
        assert_eq!(
            parse("(foo)").await.unwrap(),
            Some(Expression::Array(vec![Expression::Symbol("foo".into())]))
        );
    }
}
