use super::Value;
use alloc::string::String;
use dashmap::DashMap;
use once_cell::sync::Lazy;

static CACHE: Lazy<DashMap<String, ()>> = Lazy::new(|| Default::default());

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Symbol(*const u8);

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Symbol {}

impl From<&str> for Symbol {
    fn from(symbol: &str) -> Self {
        // TODO Can we use String keys instead to check if those keys exist or not ahead of
        // allocating heap?
        let entry = CACHE.entry(symbol.into()).or_insert_with(Default::default);

        Self(entry.key().as_ptr() as *const u8)
    }
}

impl TryFrom<Value> for Symbol {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_symbol() {
            Ok(Symbol(value.to_raw() as *const u8))
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
