use super::{
    value::{ARRAY_MASK, NIL},
    Float64, Value,
};
use alloc::{
    alloc::{alloc_zeroed, dealloc, realloc, Layout},
    vec::Vec,
};
use core::{
    fmt::{self, Display, Formatter},
    mem::forget,
    ptr::{drop_in_place, write},
};

const UNIQUE_COUNT: usize = 0;

#[derive(Debug)]
pub struct Array(u64);

#[repr(C)]
struct Header {
    count: usize,
    len: usize,
}

impl Array {
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            return Self(0);
        }

        Self(Self::mask_ptr(unsafe {
            alloc_zeroed(Self::layout(capacity))
        }))
    }

    fn mask_ptr(ptr: *const u8) -> u64 {
        let ptr = ptr as u64;

        assert!(ptr & ARRAY_MASK == 0);

        ptr | ARRAY_MASK
    }

    /// # Safety
    ///
    /// The returned array is not cloned and dropped as usual.
    pub unsafe fn from_raw(ptr: u64) -> Self {
        Self(ptr)
    }

    pub fn into_raw(self) -> u64 {
        let ptr = self.0;

        forget(self);

        ptr
    }

    pub fn get(&self, index: Value) -> Value {
        let Ok(index) = Float64::try_from(index) else { return NIL; };
        let index = index.to_f64();

        if index < 0.0 {
            NIL
        } else {
            self.get_usize(index as usize)
        }
    }

    pub fn get_usize(&self, index: usize) -> Value {
        if self.is_nil() {
            NIL
        } else if index < self.header().len {
            self.get_usize_unchecked(index)
        } else {
            NIL
        }
    }

    fn get_usize_unchecked(&self, index: usize) -> Value {
        (unsafe { &*self.element_ptr(index) }).clone()
    }

    pub fn set(self, index: Value, value: Value) -> Self {
        let Ok(index) = Float64::try_from(index) else { return Self::new(0); };
        let index = index.to_f64();

        if index < 0.0 {
            self
        } else {
            self.set_usize(index as usize, value)
        }
    }

    pub fn set_usize(mut self, index: usize, value: Value) -> Self {
        let len = index + 1;

        if self.is_nil() {
            self = Self::new(len);
            unsafe { (*self.header_mut()).len = len };
        } else if self.header().count == UNIQUE_COUNT {
            self.extend(len);
        } else {
            self = self.deep_clone(len);
        }

        self.set_usize_unchecked(index, value);

        self
    }

    pub fn is_nil(&self) -> bool {
        self.0 == 0
    }

    fn set_usize_unchecked(&mut self, index: usize, value: Value) {
        *unsafe { &mut *self.element_ptr(index) } = value;
    }

    fn extend(&mut self, len: usize) {
        if len <= self.header().len {
            return;
        }

        self.0 = Self::mask_ptr(unsafe {
            realloc(
                self.as_ptr(),
                Self::layout(self.header().len),
                Self::layout(len).size(),
            )
        });

        for index in self.header().len..len {
            unsafe { write(self.element_ptr(index), NIL) };
        }

        unsafe { &mut *self.header_mut() }.len = len;
    }

    pub fn len(&self) -> Float64 {
        (self.len_usize() as f64).into()
    }

    pub fn len_usize(&self) -> usize {
        if self.is_nil() {
            0
        } else {
            self.header().len
        }
    }

    fn deep_clone(&self, len: usize) -> Self {
        let len = self.header().len.max(len);
        let mut other = Self(unsafe { alloc_zeroed(Self::layout(len)) } as u64 | ARRAY_MASK);

        unsafe { &mut *other.header_mut() }.len = len;

        for index in 0..self.header().len {
            other.set_usize_unchecked(index, self.get_usize_unchecked(index));
        }

        other
    }

    fn header(&self) -> &Header {
        unsafe { &*self.header_mut() }
    }

    fn header_mut(&self) -> *mut Header {
        self.as_ptr() as *mut _
    }

    fn element_ptr(&self, index: usize) -> *mut Value {
        unsafe {
            self.as_ptr()
                .cast::<Header>()
                .add(1)
                .cast::<Value>()
                .add(index)
        }
    }

    fn as_ptr(&self) -> *mut u8 {
        (self.0 & !ARRAY_MASK) as usize as *mut u8
    }

    fn layout(capacity: usize) -> Layout {
        Layout::new::<Header>()
            .pad_to_align()
            .extend(Layout::array::<Value>(capacity).unwrap())
            .unwrap()
            .0
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && {
            (0..self.len_usize()).all(|index| self.get_usize(index) == other.get_usize(index))
        }
    }
}

impl Eq for Array {}

impl Clone for Array {
    fn clone(&self) -> Self {
        if !self.is_nil() {
            unsafe { &mut *self.header_mut() }.count += 1;
        }

        Self(self.0)
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        if self.is_nil() {
        } else if self.header().count == UNIQUE_COUNT {
            unsafe {
                for index in 0..self.header().len {
                    drop_in_place(self.element_ptr(index));
                }

                dealloc(self.as_ptr(), Layout::new::<Header>());
            }
        } else {
            unsafe { &mut *self.header_mut() }.count -= 1;
        }
    }
}

impl Display for Array {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "(")?;

        for index in 0..self.len_usize() {
            if index != 0 {
                write!(formatter, " ")?;
            }

            write!(formatter, "{}", self.get_usize(index))?;
        }

        write!(formatter, ")")
    }
}

impl TryFrom<Value> for Array {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_array() {
            Ok(unsafe { Array::from_raw(value.to_raw()) })
        } else {
            Err(())
        }
    }
}

impl TryFrom<&Value> for &Array {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if value.is_array() {
            let ptr = value as *const _ as *const _;

            Ok(unsafe { &*ptr })
        } else {
            Err(())
        }
    }
}

impl<const N: usize> From<[Value; N]> for Array {
    fn from(values: [Value; N]) -> Self {
        let mut array = Self::new(0);

        for (index, value) in values.into_iter().enumerate() {
            array = array.set_usize(index, value);
        }

        array
    }
}

impl From<Vec<Value>> for Array {
    fn from(values: Vec<Value>) -> Self {
        let mut array = Self::new(0);

        for (index, value) in values.into_iter().enumerate() {
            array = array.set_usize(index, value);
        }

        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn new() {
        Array::new(42);
    }

    #[test]
    fn clone() {
        #[allow(clippy::redundant_clone)]
        let _ = Array::new(42).clone();
    }

    #[test]
    fn clone_with_elements() {
        #[allow(clippy::redundant_clone)]
        let _ = Array::from([[42.0.into()].into()]).clone();
    }

    #[test]
    fn get() {
        assert_eq!(Array::new(0).get((-1.0).into()), NIL);
        assert_eq!(Array::new(0).get((-0.0).into()), NIL);
        assert_eq!(Array::new(0).get(0.0.into()), NIL);
        assert_eq!(Array::new(0).get(1.0.into()), NIL);
        assert_eq!(Array::new(1).get(0.0.into()), NIL);
    }

    #[test]
    fn display() {
        assert_eq!(&Array::from([]).to_string(), "()");
        assert_eq!(&Array::from(["foo".into()]).to_string(), "(foo)");
        assert_eq!(
            &Array::from(["foo".into(), 42.0.into()]).to_string(),
            "(foo 42)"
        );
    }

    #[test]
    fn eq() {
        assert_eq!(Array::new(0), Array::new(0));
        assert_eq!(Array::new(1), Array::new(0));
        assert_eq!(
            Array::new(0).set(0.0.into(), 42.0.into()),
            Array::new(0).set(0.0.into(), 42.0.into())
        );
        assert_ne!(
            Array::new(0).set(0.0.into(), 42.0.into()),
            Array::new(0).set(1.0.into(), 42.0.into())
        );
    }

    mod set {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn set_element() {
            let array = Array::new(0).set(0.0.into(), 42.0.into());

            assert_eq!(array.get(0.0.into()), 42.0.into());
            assert_eq!(array.get(1.0.into()), NIL);
        }

        #[test]
        fn set_element_extending_array() {
            let array = Array::new(0).set(0.0.into(), 42.0.into());

            assert_eq!(array.get(0.0.into()), 42.0.into());
            assert_eq!(array.get(1.0.into()), NIL);
        }

        #[test]
        fn set_element_extending_array_with_nil() {
            let array = Array::new(0).set(1.0.into(), 42.0.into());

            assert_eq!(array.get(0.0.into()), NIL);
            assert_eq!(array.get(1.0.into()), 42.0.into());
            assert_eq!(array.get(2.0.into()), NIL);
        }

        #[test]
        fn set_element_cloning_array() {
            let one = Array::new(0);
            let other = one.clone().set(0.0.into(), 42.0.into());

            assert_eq!(one.get(0.0.into()), NIL);
            assert_eq!(other.get(0.0.into()), 42.0.into());
        }

        #[test]
        fn set_element_without_modifying_others() {
            let one = Array::new(0).set_usize(0, NIL).set_usize(1, 13.0.into());
            let other = one.clone().set_usize(0, 42.0.into());

            assert_eq!(one.len_usize(), 2);
            assert_eq!(one.get_usize(0), NIL);
            assert_eq!(one.get_usize(1), 13.0.into());

            assert_eq!(other.len_usize(), 2);
            assert_eq!(other.get_usize(0), 42.0.into());
            assert_eq!(other.get_usize(1), 13.0.into());
        }
    }
}
