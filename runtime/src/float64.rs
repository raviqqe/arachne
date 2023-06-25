use super::Value;
use core::fmt::{self, Display, Formatter};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Float64(f64);

impl Float64 {
    #[inline]
    pub fn to_f64(self) -> f64 {
        self.0
    }
}

impl PartialEq for Float64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Float64 {}

impl Display for Float64 {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<f64> for Float64 {
    #[inline]
    fn from(number: f64) -> Self {
        Self(number)
    }
}

impl TryFrom<&Value> for Float64 {
    type Error = ();

    #[inline]
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if value.is_float64() {
            Ok(f64::from_bits(value.to_raw()).into())
        } else {
            Err(())
        }
    }
}

impl TryFrom<Value> for Float64 {
    type Error = Value;

    #[inline]
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_float64() {
            Ok(f64::from_bits(value.into_raw()).into())
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
        assert_eq!(Float64::from(0.0), Float64::from(0.0));
        assert_eq!(Float64::from(1.0), Float64::from(1.0));
        assert_ne!(Float64::from(0.0), Float64::from(1.0));
        assert_ne!(Float64::from(f64::NAN), Float64::from(f64::NAN));
    }

    #[test]
    fn ord() {
        assert!(Float64::from(0.0) < Float64::from(1.0));
        assert!(Float64::from(0.0) <= Float64::from(0.0));
        assert!(Float64::from(0.0) <= Float64::from(1.0));
        assert!(Float64::from(1.0) > Float64::from(0.0));
        assert!(Float64::from(1.0) >= Float64::from(0.0));
        assert!(Float64::from(1.0) >= Float64::from(0.0));
    }

    #[test]
    fn display() {
        assert_eq!(&Float64::from(42.0).to_string(), "42");
    }
}
