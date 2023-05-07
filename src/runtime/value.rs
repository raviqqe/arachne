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

    pub fn to_number(&self) -> Option<Number> {
        if self.is_number() {
            Some(f64::from_bits(self.0).into())
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        if self.is_array() {
            let ptr = self as *const _ as *const _;

            Some(unsafe { &*ptr })
        } else {
            None
        }
    }

    pub fn payload(&self) -> u64 {
        self.0
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(one), Some(other)) = (self.to_number(), other.to_number()) {
            one == other
        } else if let (Some(one), Some(other)) = (self.as_array(), other.as_array()) {
            one == other
        } else {
            false
        }
    }
}

impl Eq for Value {}

impl Clone for Value {
    fn clone(&self) -> Self {
        // TODO Implement a real clone.
        Self(self.0)
    }
}

impl From<f64> for Value {
    fn from(number: f64) -> Self {
        Number::from(number).into()
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
