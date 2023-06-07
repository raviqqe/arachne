use super::Value;
use core::fmt::{self, Display, Formatter};
use ordered_float::OrderedFloat;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Float64(f64);

impl Float64 {
    pub fn to_f64(self) -> f64 {
        self.0
    }
}

impl PartialEq for Float64 {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat::from(self.0) == OrderedFloat::from(other.0)
    }
}

impl Eq for Float64 {}

impl Display for Float64 {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<f64> for Float64 {
    fn from(number: f64) -> Self {
        Self(number)
    }
}

impl TryFrom<Value> for Float64 {
    type Error = Value;

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
        assert_eq!(Float64::from(f64::NAN), Float64::from(f64::NAN));
    }

    #[test]
    fn display() {
        assert_eq!(&Float64::from(42.0).to_string(), "42");
    }
}
