use super::Value;
use crate::value::SYMBOL_MASK;
use alloc::{borrow::ToOwned, boxed::Box, string::String};
use core::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    ops::Deref,
    pin::Pin,
};
use dashmap::DashMap;
use once_cell::sync::Lazy;

// TODO Inline functions.

#[allow(clippy::box_collection)]
static CACHE: Lazy<DashMap<Pin<Box<String>>, ()>> = Lazy::new(Default::default);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Symbol(u64);

impl Symbol {
    pub(crate) fn to_raw(self) -> u64 {
        self.0
    }

    pub(crate) unsafe fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub fn as_str(&self) -> &str {
        unsafe { &*((self.0 & !SYMBOL_MASK) as *const u8 as *const String) }
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Debug for Symbol {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl From<String> for Symbol {
    fn from(symbol: String) -> Self {
        let entry = CACHE
            .entry(Box::pin(symbol))
            .or_insert_with(Default::default);

        Self(entry.key().deref() as *const String as u64 | SYMBOL_MASK)
    }
}

impl From<&str> for Symbol {
    fn from(symbol: &str) -> Self {
        // TODO Can we use String keys instead to check if those keys exist or not ahead
        // of allocating heap?
        symbol.to_owned().into()
    }
}

impl TryFrom<&Value> for Symbol {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if value.is_symbol() {
            Ok(Self(value.to_raw()))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Value> for Symbol {
    type Error = Value;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_symbol() {
            Ok(Self(value.into_raw()))
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
        assert_eq!(Symbol::from("foo"), Symbol::from("foo"));
        assert_ne!(Symbol::from("foo"), Symbol::from("bar"));
    }

    #[test]
    fn ord() {
        assert!(Symbol::from("bar") < Symbol::from("foo"));
    }

    #[test]
    fn display() {
        assert_eq!(&Symbol::from("foo").to_string(), "foo");
    }
}
