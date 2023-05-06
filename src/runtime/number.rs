use super::Value;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Number(f64);

impl Number {
    pub fn to_f64(self) -> f64 {
        self.0
    }
}

impl From<f64> for Number {
    fn from(number: f64) -> Self {
        Self(number)
    }
}

impl TryFrom<Value> for Number {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_number() {
            Ok(f64::from_bits(value.payload()).into())
        } else {
            Err(())
        }
    }
}
