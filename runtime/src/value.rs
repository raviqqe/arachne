use super::{Array, Float64};
use crate::{r#type::Type, symbol::Symbol};

pub const NIL: Value = Value(0);
const EXPONENT_MASK: u64 = 0x7ff0_0000_0000_0000;
const ARRAY_SUB_MASK: u64 = 0x0004_0000_0000_0000;
const SYMBOL_SUB_MASK: u64 = 0x0002_0000_0000_0000;
pub(crate) const ARRAY_MASK: u64 = ARRAY_SUB_MASK | EXPONENT_MASK;
pub(crate) const SYMBOL_MASK: u64 = SYMBOL_SUB_MASK | EXPONENT_MASK;

#[derive(Debug)]
pub struct Value(u64);

impl Value {
    pub fn r#type(&self) -> Type {
        if self.0 & EXPONENT_MASK == 0 {
            Type::Float64
        } else if self.0 & ARRAY_MASK == ARRAY_MASK {
            Type::Array
        } else if self.0 & SYMBOL_MASK == SYMBOL_MASK {
            Type::Symbol
        } else {
            Type::Float64
        }
    }

    pub fn is_nil(&self) -> bool {
        self.0 == 0
    }

    pub fn is_array(&self) -> bool {
        self.r#type() == Type::Array
    }

    pub fn is_float64(&self) -> bool {
        self.r#type() == Type::Float64
    }

    pub fn is_symbol(&self) -> bool {
        self.r#type() == Type::Symbol
    }

    pub fn to_float64(&self) -> Option<Float64> {
        self.clone().try_into().ok()
    }

    pub fn to_symbol(&self) -> Option<Symbol> {
        self.clone().try_into().ok()
    }

    pub fn as_array(&self) -> Option<&Array> {
        self.try_into().ok()
    }

    pub fn to_raw(&self) -> u64 {
        self.0
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(one), Some(other)) = (self.to_float64(), other.to_float64()) {
            one == other
        } else if let (Some(one), Some(other)) = (self.as_array(), other.as_array()) {
            one == other
        } else if let (Some(one), Some(other)) = (self.to_symbol(), other.to_symbol()) {
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
        } else {
            Self(self.0)
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if self.is_array() {
            unsafe { Array::from_raw(self.0) };
        }
    }
}

impl From<f64> for Value {
    fn from(number: f64) -> Self {
        Float64::from(number).into()
    }
}

impl From<Array> for Value {
    fn from(array: Array) -> Self {
        Self(array.into_raw())
    }
}

impl From<Float64> for Value {
    fn from(number: Float64) -> Self {
        Self(number.to_f64().to_bits())
    }
}

impl From<Symbol> for Value {
    fn from(symbol: Symbol) -> Self {
        Self(symbol.to_raw())
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
        assert!(Value::from(f64::NAN).is_float64());
        assert!(Value::from(-f64::NAN).is_float64());
        assert!(Value::from(-0.0 / 0.0).is_float64());
    }

    #[test]
    fn zero_division() {
        assert!(Value::from(-0.0).is_float64());
        assert!(Value::from(-1.0).is_float64());
        assert!(Value::from(1.0 / 0.0).is_float64());
        assert!(Value::from(-1.0 / 0.0).is_float64());
    }

    mod clone {
        use super::*;

        #[test]
        fn clone_float64() {
            let _ = Value::from(0.0);
        }

        #[test]
        fn clone_symbols() {
            let _ = Value::from(Symbol::from("foo"));
        }

        #[test]
        fn clone_array() {
            let _ = Value::from(Array::new(42));
        }
    }

    #[test]
    fn compare_float64() {
        assert_eq!(Value::from(0.0), Value::from(0.0));
        assert_eq!(Value::from(1.0), Value::from(1.0));
        assert_ne!(Value::from(0.0), Value::from(1.0));
        assert_eq!(Value::from(f64::NAN), Value::from(f64::NAN));
    }

    #[test]
    fn compare_symbol() {
        assert_eq!(
            Value::from(Symbol::from("foo")),
            Value::from(Symbol::from("foo"))
        );
        assert_ne!(
            Value::from(Symbol::from("foo")),
            Value::from(Symbol::from("bar"))
        );
    }

    #[test]
    fn compare_arrays() {
        // TODO Compare arrays.
    }
}
