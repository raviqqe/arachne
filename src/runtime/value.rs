use super::{Array, Number};

pub const NIL: Value = Value(0);
pub const ARRAY_MASK: u64 = 1 << 63;

#[derive(Debug)]
pub struct Value(u64);

impl Value {
    pub fn is_number(&self) -> bool {
        self.0 & ARRAY_MASK == 0
    }

    pub fn is_array(&self) -> bool {
        self.0 & ARRAY_MASK != 0
    }

    pub fn payload(&self) -> u64 {
        self.0
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        // TODO Implement a real clone.
        Self(self.0)
    }
}

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
