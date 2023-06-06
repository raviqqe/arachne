use super::Value;
use crate::value::SYMBOL_MASK;
use alloc::{borrow::ToOwned, string::String};
use dashmap::DashMap;
use once_cell::sync::Lazy;

static CACHE: Lazy<DashMap<String, ()>> = Lazy::new(Default::default);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Symbol(u64);

impl Symbol {
    pub(crate) fn to_raw(self) -> u64 {
        self.0
    }
}

impl From<String> for Symbol {
    fn from(symbol: String) -> Self {
        let entry = CACHE.entry(symbol).or_insert_with(Default::default);

        Self(entry.key().as_ptr() as u64 | SYMBOL_MASK)
    }
}

impl From<&str> for Symbol {
    fn from(symbol: &str) -> Self {
        // TODO Can we use String keys instead to check if those keys exist or not ahead
        // of allocating heap?
        symbol.to_owned().into()
    }
}

impl TryFrom<Value> for Symbol {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_symbol() {
            Ok(Symbol(value.to_raw()))
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
        assert_eq!(Symbol::from("foo"), Symbol::from("foo"));
        assert_ne!(Symbol::from("foo"), Symbol::from("bar"));
    }
}
