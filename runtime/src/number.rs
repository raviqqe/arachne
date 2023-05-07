use super::Value;
use ordered_float::OrderedFloat;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Number(f64);

impl Number {
    pub fn to_f64(self) -> f64 {
        self.0
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat::from(self.0) == OrderedFloat::from(other.0)
    }
}

impl Eq for Number {}

impl From<f64> for Number {
    fn from(number: f64) -> Self {
        Self(number)
    }
}

impl TryFrom<Value> for Number {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_number() {
            Ok(f64::from_bits(value.to_raw()).into())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        assert_eq!(Number::from(0.0), Number::from(0.0));
        assert_eq!(Number::from(1.0), Number::from(1.0));
        assert_ne!(Number::from(0.0), Number::from(1.0));
        assert_eq!(Number::from(f64::NAN), Number::from(f64::NAN));
    }
}
