use super::Value;
use crate::value::SYMBOL_MASK;
use core::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Integer32(u64);

impl Integer32 {
    pub(crate) fn to_raw(self) -> u64 {
        self.0
    }

    pub fn to_i32(&self) -> i32 {
        let mut buffer = [0u8; 4];

        buffer.copy_from_slice(&self.0.to_le_bytes()[..2]);

        i32::from_le_bytes(buffer)
    }
}

impl Debug for Integer32 {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_i32())
    }
}

impl Display for Integer32 {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_i32())
    }
}

impl From<i32> for Integer32 {
    fn from(number: i32) -> Self {
        Self(number as u32 as u64 | SYMBOL_MASK)
    }
}

impl TryFrom<&Value> for Integer32 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if value.is_integer32() {
            Ok(Self(value.to_raw()))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Value> for Integer32 {
    type Error = Value;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_symbol() {
            Ok(Self(value.into_raw()))
        } else {
            Err(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn eq() {
        assert_eq!(Integer32::from(42i32), Integer32::from(42i32));
    }

    #[test]
    fn display() {
        assert_eq!(&Integer32::from("foo").to_string(), "foo");
    }
}
