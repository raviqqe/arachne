use super::error::ParseError;
use crate::expression::Expression;
use async_recursion::async_recursion;
use std::{io, marker::Unpin};
use tokio::io::AsyncReadExt;

const SPECIAL_CHARACTERS: &str = "(); \t\n";
const SYMBOL_CAPACITY: usize = 8;
const ARRAY_CAPACITY: usize = 8;
const BUFFER_CAPACITY: usize = 8;

pub struct Parser {
    buffer: String,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(BUFFER_CAPACITY),
        }
    }

    pub async fn parse_expression(
        &mut self,
        reader: &mut (impl AsyncReadExt + Unpin),
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
        reader: &mut (impl AsyncReadExt + Unpin),
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
        reader: &mut (impl AsyncReadExt + Unpin),
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
        reader: &mut (impl AsyncReadExt + Unpin),
    ) -> Result<(), ParseError> {
        while !matches!(self.read_character(reader).await?, Some('\n') | None) {}

        Ok(())
    }

    // TODO Support UTF-8.
    async fn read_character(
        &mut self,
        reader: &mut (impl AsyncReadExt + Unpin),
    ) -> Result<Option<char>, ParseError> {
        if let Some(character) = self.buffer.pop() {
            Ok(Some(character))
        } else {
            match reader.read_u8().await {
                Ok(byte) => Ok(Some(char::try_from(byte)?)),
                Err(error) => {
                    if error.kind() == io::ErrorKind::UnexpectedEof {
                        Ok(None)
                    } else {
                        Err(error.into())
                    }
                }
            }
        }
    }
}
