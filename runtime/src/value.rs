use super::{Array, Float64};
use crate::{integer32::Integer32, r#type::Type, symbol::Symbol, Closure, TypedValue};
use alloc::{string::String, vec::Vec};
use core::{
    fmt::{self, Display, Formatter},
    mem::forget,
};

pub const NIL: Value = Value(0);
const EXPONENT_MASK: u64 = 0x7ff0 << 48;
const TYPE_SUB_MASK: usize = 0b111;
const INTEGER32_SUB_MASK: usize = 0b001;
const SYMBOL_SUB_MASK: usize = 0b011;
const CLOSURE_SUB_MASK: usize = 0b010;
const ARRAY_SUB_MASK: usize = 0b100;
const TYPE_MASK_OFFSET: usize = 48;

const fn build_mask(sub_mask: usize) -> u64 {
    ((sub_mask as u64) << TYPE_MASK_OFFSET) | EXPONENT_MASK
}

pub(crate) const TYPE_MASK: u64 = build_mask(TYPE_SUB_MASK);
pub(crate) const ARRAY_MASK: u64 = build_mask(ARRAY_SUB_MASK);
pub(crate) const CLOSURE_MASK: u64 = build_mask(CLOSURE_SUB_MASK);
pub(crate) const SYMBOL_MASK: u64 = build_mask(SYMBOL_SUB_MASK);
pub(crate) const INTEGER32_MASK: u64 = build_mask(INTEGER32_SUB_MASK);

#[derive(Debug)]
pub struct Value(u64);

impl Value {
    pub fn r#type(&self) -> Type {
        if self.0 & EXPONENT_MASK != EXPONENT_MASK {
            return Type::Float64;
        }

        match ((self.0 >> TYPE_MASK_OFFSET) as usize) & TYPE_SUB_MASK {
            INTEGER32_SUB_MASK => Type::Integer32,
            SYMBOL_SUB_MASK => Type::Symbol,
            CLOSURE_SUB_MASK => Type::Closure,
            ARRAY_SUB_MASK => Type::Array,
            _ => Type::Float64,
        }
    }

    pub fn is_nil(&self) -> bool {
        self.0 == 0
    }

    pub fn is_array(&self) -> bool {
        self.is_nil() || self.r#type() == Type::Array
    }

    pub fn is_float64(&self) -> bool {
        self.is_nil() || self.r#type() == Type::Float64
    }

    pub fn is_integer32(&self) -> bool {
        self.is_nil() || self.r#type() == Type::Integer32
    }

    pub fn is_closure(&self) -> bool {
        self.r#type() == Type::Closure
    }

    pub fn is_symbol(&self) -> bool {
        self.r#type() == Type::Symbol
    }

    pub fn to_float64(&self) -> Option<Float64> {
        self.try_into().ok()
    }

    pub fn to_integer32(&self) -> Option<Integer32> {
        self.try_into().ok()
    }

    pub fn to_symbol(&self) -> Option<Symbol> {
        self.try_into().ok()
    }

    pub fn into_array(self) -> Option<Array> {
        self.try_into().ok()
    }

    pub fn as_array(&self) -> Option<&Array> {
        self.try_into().ok()
    }

    pub fn into_closure(self) -> Option<Closure> {
        self.try_into().ok()
    }

    pub fn as_closure(&self) -> Option<&Closure> {
        self.try_into().ok()
    }

    pub fn into_typed(self) -> Option<TypedValue> {
        if self.is_nil() {
            None
        } else {
            Some(match self.r#type() {
                Type::Array => TypedValue::Array(self.try_into().unwrap()),
                Type::Closure => TypedValue::Closure(self.try_into().unwrap()),
                Type::Float64 => TypedValue::Float64(self.try_into().unwrap()),
                Type::Integer32 => TypedValue::Integer32(self.try_into().unwrap()),
                Type::Symbol => TypedValue::Symbol(self.try_into().unwrap()),
            })
        }
    }

    pub fn into_raw(self) -> u64 {
        let raw = self.0;

        forget(self);

        raw
    }

    pub fn to_raw(&self) -> u64 {
        self.0
    }

    /// # Safety
    ///
    /// The raw content must be valid and moved into the new value.
    pub unsafe fn from_raw(value: u64) -> Self {
        Self(value)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(one), Some(other)) = (self.as_array(), other.as_array()) {
            one == other
        } else if let (Some(one), Some(other)) = (self.to_float64(), other.to_float64()) {
            one == other
        } else if let (Some(one), Some(other)) = (self.to_integer32(), other.to_integer32()) {
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
        } else if let Some(closure) = self.as_closure() {
            closure.clone().into()
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

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if self.is_nil() {
            write!(formatter, "()")
        } else if let Some(number) = self.to_float64() {
            write!(formatter, "{}", number)
        } else if let Some(closure) = self.as_closure() {
            write!(formatter, "{}", closure)
        } else if let Some(symbol) = self.to_symbol() {
            write!(formatter, "{}", symbol)
        } else if let Some(array) = self.as_array() {
            write!(formatter, "{}", array)
        } else {
            unreachable!()
        }
    }
}

impl From<Array> for Value {
    fn from(array: Array) -> Self {
        Self(array.into_raw())
    }
}

impl From<Closure> for Value {
    fn from(closure: Closure) -> Self {
        Self(closure.into_raw())
    }
}

impl From<Float64> for Value {
    fn from(number: Float64) -> Self {
        Self(number.to_f64().to_bits())
    }
}

impl From<Integer32> for Value {
    fn from(number: Integer32) -> Self {
        Self(number.to_raw())
    }
}

impl From<Symbol> for Value {
    fn from(symbol: Symbol) -> Self {
        Self(symbol.to_raw())
    }
}

impl From<f64> for Value {
    fn from(number: f64) -> Self {
        Float64::from(number).into()
    }
}

impl From<i32> for Value {
    fn from(number: i32) -> Self {
        Integer32::from(number).into()
    }
}

impl From<u32> for Value {
    fn from(number: u32) -> Self {
        Integer32::from(number).into()
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Symbol::from(value).into()
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Symbol::from(value).into()
    }
}

impl<const N: usize> From<[Value; N]> for Value {
    fn from(values: [Value; N]) -> Self {
        Array::from(values).into()
    }
}

impl From<Vec<Value>> for Value {
    fn from(values: Vec<Value>) -> Self {
        Array::from(values).into()
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
    fn nil_type_check() {
        assert!(NIL.is_nil());
        assert!(NIL.is_array());
        assert!(!NIL.is_closure());
        assert!(NIL.is_float64());
        assert!(!NIL.is_symbol());
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
    fn compare_integer32() {
        assert_eq!(Value::from(0i32), Value::from(0i32));
        assert_eq!(Value::from(42i32), Value::from(42i32));
        assert_eq!(Value::from(-42i32), Value::from(-42i32));
        assert_ne!(Value::from(0i32), Value::from(1i32));
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
        assert_eq!(Value::from(Array::new(0)), Value::from(0.0));
        assert_ne!(Value::from(Array::new(0)), Value::from(42.0));
        assert_ne!(Value::from(Array::new(0)), Value::from(Symbol::from("foo")));
        assert_eq!(Value::from(Array::new(0)), Value::from(Array::new(0)));
        assert_eq!(
            Value::from(Array::new(0).set(0.0.into(), 42.0.into())),
            Value::from(Array::new(0).set(0.0.into(), 42.0.into()))
        );
        assert_ne!(
            Value::from(Array::new(0).set(0.0.into(), 42.0.into())),
            Value::from(Array::new(0).set(1.0.into(), 42.0.into()))
        );
    }
}
