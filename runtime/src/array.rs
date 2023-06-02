use super::{
    value::{ARRAY_MASK, NIL},
    Float64, Value,
};
use alloc::alloc::{alloc_zeroed, dealloc, realloc, Layout};
use core::mem::{align_of, forget, size_of};

const UNIQUE_COUNT: usize = 0;
const ELEMENT_SIZE: usize = size_of::<Value>();
const ALIGNMENT: usize = align_of::<Value>();

#[derive(Debug)]
pub struct Array(u64);

#[repr(C)]
struct Header {
    count: usize,
    len: usize,
}

impl Array {
    // TODO Handle zero capacity properly.
    pub fn new(capacity: usize) -> Self {
        let ptr = unsafe { alloc_zeroed(Self::layout(capacity)) } as usize as u64;

        assert!(ptr & ARRAY_MASK == 0);

        Self(ptr | ARRAY_MASK)
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
        if index < self.header().len {
            self.get_usize_unchecked(index)
        } else {
            NIL
        }
    }

    fn get_usize_unchecked(&self, index: usize) -> Value {
        (unsafe { &*self.element_ptr(index) }).clone()
    }

    pub fn set(self, index: Value, value: Value) -> Value {
        let Ok(index) = Float64::try_from(index) else { return NIL; };
        let index = index.to_f64();

        if index < 0.0 {
            self.into()
        } else {
            self.set_usize(index as usize, value)
        }
    }

    pub fn set_usize(mut self, index: usize, value: Value) -> Value {
        let len = index + 1;

        if self.header().count == UNIQUE_COUNT {
            self.extend(len);
        } else {
            self = self.deep_clone(len);
        }

        self.set_usize_unchecked(index, value);

        self.into()
    }

    fn set_usize_unchecked(&mut self, index: usize, value: Value) {
        *unsafe { &mut *self.element_ptr(index) } = value;
    }

    fn extend(&mut self, len: usize) {
        if len <= self.header().len {
            return;
        }

        self.0 = unsafe {
            realloc(
                self.as_ptr(),
                Self::layout(self.header().len),
                Self::layout(len).size(),
            )
        } as u64
            | ARRAY_MASK;

        unsafe { &mut *self.header_mut() }.len = len;
    }

    pub fn len(&self) -> Value {
        Float64::from(self.header().len as f64).into()
    }

    fn deep_clone(&mut self, len: usize) -> Self {
        let len = self.header().len.max(len);
        let ptr = unsafe { alloc_zeroed(Self::layout(len)) };

        for index in 0..self.header().len {
            self.set_usize_unchecked(index, self.get_usize_unchecked(index));
        }

        let other = Self(ptr as u64 | ARRAY_MASK);

        unsafe { &mut *other.header_mut() }.len = len;

        other
    }

    fn header(&self) -> &Header {
        unsafe { &*self.header_mut() }
    }

    fn header_mut(&self) -> *mut Header {
        self.as_ptr() as *mut _
    }

    fn element_ptr(&self, index: usize) -> *mut Value {
        ((self.as_ptr() as usize + Layout::new::<Header>().size()) + index * ELEMENT_SIZE)
            as *mut Value
    }

    fn as_ptr(&self) -> *mut u8 {
        (self.0 & !ARRAY_MASK) as usize as *mut u8
    }

    fn layout(capacity: usize) -> Layout {
        Layout::new::<Header>()
            .extend(Layout::from_size_align(ELEMENT_SIZE * capacity, ALIGNMENT).unwrap())
            .unwrap()
            .0
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
    }
}

impl Eq for Array {}

impl Clone for Array {
    fn clone(&self) -> Self {
        unsafe { &mut *self.header_mut() }.count += 1;

        Self(self.0)
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        if self.header().count == 0 {
            // TODO Drop elements.
            unsafe { dealloc(self.as_ptr(), Layout::new::<Header>()) }
        } else {
            unsafe { &mut *self.header_mut() }.count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        Array::new(42);
    }

    #[test]
    fn clone() {
        let _ = Array::new(42);
    }

    #[test]
    fn get() {
        assert_eq!(Array::new(0).get((-1.0).into()), NIL);
        assert_eq!(Array::new(0).get((-0.0).into()), NIL);
        assert_eq!(Array::new(0).get(0.0.into()), NIL);
        assert_eq!(Array::new(0).get(1.0.into()), NIL);
        assert_eq!(Array::new(1).get(0.0.into()), NIL);
    }

    mod set {
        use super::*;

        #[test]
        fn set_element() {
            let value = Array::new(0).set(0.0.into(), 42.0.into());
            let array = value.as_array().unwrap();

            assert_eq!(array.get(0.0.into()), 42.0.into());
            assert_eq!(array.get(1.0.into()), NIL);
        }

        #[test]
        fn set_element_extending_array() {
            let value = Array::new(0).set(0.0.into(), 42.0.into());
            let array = value.as_array().unwrap();

            assert_eq!(array.get(0.0.into()), 42.0.into());
            assert_eq!(array.get(1.0.into()), NIL);
        }

        #[test]
        fn set_element_extending_array_with_nil() {
            let value = Array::new(0).set(1.0.into(), 42.0.into());
            let array = value.as_array().unwrap();

            assert_eq!(array.get(0.0.into()), NIL);
            assert_eq!(array.get(1.0.into()), 42.0.into());
            assert_eq!(array.get(2.0.into()), NIL);
        }

        #[test]
        fn set_element_cloning_array() {
            let one = Array::new(0);
            let value = one.clone().set(0.0.into(), 42.0.into());
            let other = value.as_array().unwrap();

            assert_eq!(one.get(0.0.into()), NIL);
            assert_eq!(other.get(0.0.into()), 42.0.into());
        }
    }
}
