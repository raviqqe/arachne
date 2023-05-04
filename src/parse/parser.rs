use super::error::ParseError;
use crate::expression::Expression;
use async_recursion::async_recursion;
use std::{io, marker::Unpin};
use tokio::io::AsyncBufReadExt;

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

    pub async fn parse_expression(
        &mut self,
        reader: &mut (impl AsyncBufReadExt + Unpin),
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
    async fn parse_parentheses(
        &mut self,
        reader: &mut (impl AsyncBufReadExt + Unpin),
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

    async fn parse_symbol(
        &mut self,
        reader: &mut (impl AsyncBufReadExt + Unpin),
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

    async fn parse_comment(
        &mut self,
        reader: &mut (impl AsyncBufReadExt + Unpin),
    ) -> Result<(), ParseError> {
        while !matches!(self.read_character(reader).await?, Some('\n') | None) {}

        Ok(())
    }

    // TODO Support UTF-8.
    async fn read_character(
        &mut self,
        reader: &mut (impl AsyncBufReadExt + Unpin),
    ) -> Result<Option<char>, ParseError> {
        if self.buffer.len() == self.offset {
            if let Err(error) = reader.read_line(&mut self.buffer).await {
                if error.kind() != io::ErrorKind::UnexpectedEof {
                    return Err(error.into());
                }
            }
        }

        self.offset += 1;

        Ok(self.buffer.chars().nth(self.offset - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{expression::Expression, parse::error::ParseError};

    async fn parse(string: &str) -> Result<Option<Expression>, ParseError> {
        let mut parser = Parser::new();

        parser.parse_expression(&mut string.as_bytes()).await
    }

    #[tokio::test]
    async fn parse_expression() {
        assert_eq!(
            parse("foo").await.unwrap(),
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
