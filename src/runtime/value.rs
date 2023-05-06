use super::{Array, Number};

#[repr(transparent)]
pub struct Value(u64);

impl Value {}

impl From<Number> for Value {
    fn from(number: Number) -> Self {
        Self(number.to_f64().to_bits())
    }
}

impl From<Array> for Value {
    fn from(array: Array) -> Self {
        Self(array.to_u64())
    }
}
