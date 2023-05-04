mod error;

use crate::expression::Expression;
use async_recursion::async_recursion;
use error::ParseError;
use std::marker::Unpin;
use tokio::io::AsyncReadExt;

const SPECIAL_CHARACTERS: &[u8] = b"(); \t";
const SYMBOL_CAPACITY: usize = 8;
const ARRAY_CAPACITY: usize = 8;

// TODO Support UTF-8.
pub async fn parse_expression(
    reader: &mut (impl AsyncReadExt + Unpin),
) -> Result<Option<Expression>, ParseError> {
    loop {
        match reader.read_u8().await? {
            b'(' => return Ok(Some(parse_parentheses(reader).await?)),
            b')' => return Err(ParseError::ClosedParenthesis),
            b';' => {
                parse_comment(reader).await?;
                continue;
            }
            b' ' | b'\t' => continue,
            character => return Ok(Some(parse_symbol(reader, character).await?)),
        }
    }
}

#[async_recursion(?Send)]
async fn parse_parentheses(
    reader: &mut (impl AsyncReadExt + Unpin),
) -> Result<Expression, ParseError> {
    let mut vector = Vec::with_capacity(ARRAY_CAPACITY);

    loop {
        match parse_expression(reader).await {
            Err(ParseError::ClosedParenthesis) => return Ok(Expression::Array(vector)),
            Err(error) => return Err(error),
            Ok(None) => return Err(ParseError::EndOfFile),
            Ok(Some(expression)) => vector.push(expression),
        }
    }
}

async fn parse_symbol(
    reader: &mut (impl AsyncReadExt + Unpin),
    character: u8,
) -> Result<Expression, ParseError> {
    let mut vector = Vec::with_capacity(SYMBOL_CAPACITY);

    vector.push(character);

    loop {
        let character = reader.read_u8().await?;

        if SPECIAL_CHARACTERS.contains(&character) {
            return Ok(Expression::Symbol(String::from_utf8(vector)?));
        }
    }
}

async fn parse_comment(reader: &mut (impl AsyncReadExt + Unpin)) -> Result<(), ParseError> {
    while reader.read_u8().await? != b'\n' {}

    Ok(())
}
