use super::{Array, Number};

pub const NIL: Value = Value(0);
const EXPONENT_MASK: u64 = 0x7ff0_0000_0000_0000;
pub const ARRAY_MASK: u64 = 0x1 | EXPONENT_MASK;

#[derive(Debug)]
pub struct Value(u64);

impl Value {
    pub fn is_number(&self) -> bool {
        !self.is_array()
    }

    pub fn is_array(&self) -> bool {
        self.0 & ARRAY_MASK == ARRAY_MASK
    }

    pub fn to_number(&self) -> Option<Number> {
        if self.is_number() {
            self.clone().try_into().ok()
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

    pub fn to_raw(&self) -> u64 {
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
        if let Some(array) = self.as_array() {
            array.clone().into()
        } else if self.is_number() {
            Self(self.0)
        } else {
            unreachable!()
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if self.is_array() {
            unsafe { Array::from_raw(self.0) };
        } else if !self.is_number() {
            unreachable!()
        }
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
        Self(array.into_raw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nil() {
        assert_eq!(NIL, Value::from(0.0));
    }

    #[test]
    fn nan() {
        assert!(Value::from(f64::NAN).is_number());
    }

    #[test]
    fn zero_division() {
        assert!(Value::from(-0.0).is_number());
        assert!(Value::from(-1.0).is_number());
        assert!(Value::from(1.0 / 0.0).is_number());
        assert!(Value::from(-1.0 / 0.0).is_number());
        assert!(Value::from(0.0 / 0.0).is_number());
        assert!(Value::from(-0.0 / 0.0).is_number());
    }

    mod clone {
        use super::*;

        #[test]
        fn clone_number() {
            let _ = Value::from(0.0);
        }

        #[test]
        fn clone_array() {
            let _ = Value::from(Array::new(42));
        }
    }

    #[test]
    fn compare_numbers() {
        assert_eq!(Value::from(0.0), Value::from(0.0));
        assert_eq!(Value::from(1.0), Value::from(1.0));
        assert_ne!(Value::from(0.0), Value::from(1.0));
        assert_eq!(Value::from(f64::NAN), Value::from(f64::NAN));
    }

    #[test]
    fn compare_arrays() {
        // TODO Compare arrays.
    }
}
