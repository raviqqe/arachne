use super::Value;
use crate::value::INTEGER32_MASK;
use core::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    mem::size_of,
};

// TODO Inline functions.

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Integer32(u64);

impl Integer32 {
    pub(crate) fn to_raw(self) -> u64 {
        self.0
    }

    pub(crate) unsafe fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub fn to_i32(self) -> i32 {
        const SIZE: usize = size_of::<u32>();
        let mut buffer = [0u8; SIZE];

        buffer.copy_from_slice(&self.0.to_le_bytes()[..SIZE]);

        i32::from_le_bytes(buffer)
    }
}

impl PartialOrd for Integer32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_i32().cmp(&other.to_i32())
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
        (number as u32).into()
    }
}

impl From<u32> for Integer32 {
    fn from(number: u32) -> Self {
        Self(nonbox::r#box(number as u64 | INTEGER32_MASK).to_bits())
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
        if value.is_integer32() {
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
        assert_eq!(Integer32::from(42), Integer32::from(42));
    }

    #[test]
    fn ord() {
        assert!(Integer32::from(0) < Integer32::from(1));
        assert!(Integer32::from(0) <= Integer32::from(0));
        assert!(Integer32::from(0) <= Integer32::from(1));
        assert!(Integer32::from(1) > Integer32::from(0));
        assert!(Integer32::from(1) >= Integer32::from(0));
        assert!(Integer32::from(1) >= Integer32::from(0));
    }

    #[test]
    fn convert() {
        assert_eq!(Integer32::from(42).to_i32(), 42);
        assert_eq!(Integer32::from(-42).to_i32(), -42);
        assert_eq!(Integer32::from(u32::MAX).to_i32(), -1);
        assert_eq!(Integer32::from(i32::MIN).to_i32(), i32::MIN);
    }

    #[test]
    fn display() {
        assert_eq!(&Integer32::from(42).to_string(), "42");
    }
}
