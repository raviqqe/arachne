use crate::expression::Expression;
use std::error::Error;
use tokio::io::AsyncRead;

pub async fn parse_expression(
    reader: &mut impl AsyncRead,
) -> Result<Option<Expression>, Box<dyn Error>> {
    todo!()
}
