use super::{Array, Float64};
use crate::{
    integer32::Integer32, r#type::Type, symbol::Symbol, typed_value::TypedValueRef, Closure,
    TypedValue,
};
use alloc::{string::String, vec::Vec};
use core::{
    cmp::Ordering,
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

pub(crate) const ARRAY_MASK: u64 = build_mask(ARRAY_SUB_MASK);
pub(crate) const CLOSURE_MASK: u64 = build_mask(CLOSURE_SUB_MASK);
pub(crate) const SYMBOL_MASK: u64 = build_mask(SYMBOL_SUB_MASK);
pub(crate) const INTEGER32_MASK: u64 = build_mask(INTEGER32_SUB_MASK);

#[derive(Debug)]
pub struct Value(u64);

impl Value {
    #[inline]
    pub const fn r#type(&self) -> Type {
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

    #[inline]
    pub const fn is_nil(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        self.is_nil() || self.r#type() == Type::Array
    }

    #[inline]
    pub fn is_float64(&self) -> bool {
        self.is_nil() || self.r#type() == Type::Float64
    }

    #[inline]
    pub fn is_integer32(&self) -> bool {
        self.is_nil() || self.r#type() == Type::Integer32
    }

    #[inline]
    pub fn is_closure(&self) -> bool {
        self.is_nil() || self.r#type() == Type::Closure
    }

    #[inline]
    pub fn is_symbol(&self) -> bool {
        self.r#type() == Type::Symbol
    }

    #[inline]
    pub fn to_float64(&self) -> Option<Float64> {
        self.try_into().ok()
    }

    #[inline]
    pub fn to_integer32(&self) -> Option<Integer32> {
        self.try_into().ok()
    }

    #[inline]
    pub fn to_symbol(&self) -> Option<Symbol> {
        self.try_into().ok()
    }

    #[inline]
    pub fn into_array(self) -> Option<Array> {
        self.try_into().ok()
    }

    #[inline]
    pub fn as_array(&self) -> Option<&Array> {
        self.try_into().ok()
    }

    #[inline]
    pub fn into_closure(self) -> Option<Closure> {
        self.try_into().ok()
    }

    #[inline]
    pub fn as_closure(&self) -> Option<&Closure> {
        self.try_into().ok()
    }

    #[inline]
    pub fn as_typed(&self) -> Option<TypedValueRef> {
        if self.is_nil() {
            None
        } else {
            Some(match self.r#type() {
                Type::Array => TypedValueRef::Array(unsafe { &*(self as *const _ as *const _) }),
                Type::Closure => {
                    TypedValueRef::Closure(unsafe { &*(self as *const _ as *const _) })
                }
                Type::Float64 => TypedValueRef::Float64(Float64::from(f64::from_bits(self.0))),
                Type::Integer32 => TypedValueRef::Integer32(unsafe { Integer32::from_raw(self.0) }),
                Type::Symbol => TypedValueRef::Symbol(unsafe { Symbol::from_raw(self.0) }),
            })
        }
    }

    #[inline]
    pub fn into_typed(self) -> Option<TypedValue> {
        if self.is_nil() {
            None
        } else {
            Some(match self.r#type() {
                Type::Array => TypedValue::Array(unsafe { Array::from_raw(self.0) }),
                Type::Closure => TypedValue::Closure(unsafe { Closure::from_raw(self.0) }),
                Type::Float64 => TypedValue::Float64(Float64::from(f64::from_bits(self.0))),
                Type::Integer32 => TypedValue::Integer32(unsafe { Integer32::from_raw(self.0) }),
                Type::Symbol => TypedValue::Symbol(unsafe { Symbol::from_raw(self.0) }),
            })
        }
    }

    #[inline]
    pub(crate) fn into_raw(self) -> u64 {
        let raw = self.0;

        forget(self);

        raw
    }

    #[inline]
    pub(crate) fn to_raw(&self) -> u64 {
        self.0
    }
}

impl PartialEq for Value {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if let Some(value) = self.as_typed() {
            match value {
                TypedValueRef::Float64(one) => Some(one) == other.to_float64(),
                TypedValueRef::Closure(_) => false,
                TypedValueRef::Integer32(one) => Some(one) == other.to_integer32(),
                TypedValueRef::Array(one) => Some(one) == other.as_array(),
                TypedValueRef::Symbol(one) => Some(one) == other.to_symbol(),
            }
        } else {
            other.is_nil()
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(value) = self.as_typed() {
            match value {
                TypedValueRef::Float64(one) => {
                    other.to_float64().and_then(|other| one.partial_cmp(&other))
                }
                TypedValueRef::Closure(_) => None,
                TypedValueRef::Integer32(one) => other
                    .to_integer32()
                    .and_then(|other| one.partial_cmp(&other)),
                TypedValueRef::Array(one) => {
                    other.as_array().and_then(|other| one.partial_cmp(other))
                }
                TypedValueRef::Symbol(one) => {
                    other.to_symbol().and_then(|other| one.partial_cmp(&other))
                }
            }
        } else {
            Some(if other.is_nil() {
                Ordering::Equal
            } else {
                Ordering::Less
            })
        }
    }
}

impl Clone for Value {
    #[inline]
    fn clone(&self) -> Self {
        match self.as_typed() {
            None => NIL,
            Some(TypedValueRef::Array(array)) => array.clone().into(),
            Some(TypedValueRef::Closure(closure)) => closure.clone().into(),
            Some(
                TypedValueRef::Float64(_) | TypedValueRef::Integer32(_) | TypedValueRef::Symbol(_),
            ) => Self(self.0),
        }
    }
}

impl Drop for Value {
    #[inline]
    fn drop(&mut self) {
        match self.r#type() {
            Type::Array => unsafe {
                Array::from_raw(self.0);
            },
            Type::Closure => unsafe {
                Closure::from_raw(self.0);
            },
            Type::Float64 | Type::Integer32 | Type::Symbol => {}
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
    #[inline]
    fn from(array: Array) -> Self {
        Self(array.into_raw())
    }
}

impl From<Closure> for Value {
    #[inline]
    fn from(closure: Closure) -> Self {
        Self(closure.into_raw())
    }
}

impl From<Float64> for Value {
    #[inline]
    fn from(number: Float64) -> Self {
        Self(number.to_f64().to_bits())
    }
}

impl From<Integer32> for Value {
    #[inline]
    fn from(number: Integer32) -> Self {
        Self(number.to_raw())
    }
}

impl From<Symbol> for Value {
    #[inline]
    fn from(symbol: Symbol) -> Self {
        Self(symbol.to_raw())
    }
}

impl From<f64> for Value {
    #[inline]
    fn from(number: f64) -> Self {
        Float64::from(number).into()
    }
}

impl From<i32> for Value {
    #[inline]
    fn from(number: i32) -> Self {
        Integer32::from(number).into()
    }
}

impl From<u32> for Value {
    #[inline]
    fn from(number: u32) -> Self {
        Integer32::from(number).into()
    }
}

impl From<String> for Value {
    #[inline]
    fn from(value: String) -> Self {
        Symbol::from(value).into()
    }
}

impl From<&str> for Value {
    #[inline]
    fn from(value: &str) -> Self {
        Symbol::from(value).into()
    }
}

impl<const N: usize> From<[Value; N]> for Value {
    #[inline]
    fn from(values: [Value; N]) -> Self {
        Array::from(values).into()
    }
}

impl From<Vec<Value>> for Value {
    #[inline]
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
        assert!(NIL.is_closure());
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
