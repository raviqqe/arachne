use super::{
    value::{ARRAY_MASK, NIL},
    Number, Value,
};
use alloc::alloc::{alloc, dealloc, Layout};
use core::mem::{align_of, forget, size_of};

#[derive(Debug)]
pub struct Array(u64);

#[repr(C)]
struct Header {
    count: usize,
    length: usize,
}

impl Array {
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::new::<Header>()
            .extend(
                Layout::from_size_align(size_of::<Value>() * capacity, align_of::<Value>())
                    .unwrap(),
            )
            .unwrap()
            .0;
        let this = Self(unsafe { alloc(layout) } as usize as u64 | ARRAY_MASK);

        forget(this.clone());

        this
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
        let Ok(index) = Number::try_from(index) else { return NIL; };
        let index = index.to_f64() as usize;

        if index < self.len_usize() {
            let ptr = (self.element_ptr() as usize + size_of::<Value>()) as *const Value;

            (unsafe { &*ptr }).clone()
        } else {
            NIL
        }
    }

    pub fn set(&self, _index: Value, _value: Value) -> Value {
        todo!()
    }

    pub fn len(&self) -> Value {
        Number::from(self.len_usize() as f64).into()
    }

    fn len_usize(&self) -> usize {
        self.header().length
    }

    fn header(&self) -> &Header {
        unsafe { &*self.header_mut() }
    }

    fn header_mut(&self) -> *mut Header {
        self.as_ptr() as *mut _
    }

    fn element_ptr(&self) -> *mut u8 {
        (self.ptr_usize() + Layout::new::<Header>().size()) as *mut u8
    }

    fn as_ptr(&self) -> *mut u8 {
        self.ptr_usize() as *mut u8
    }

    fn ptr_usize(&self) -> usize {
        (self.0 & !ARRAY_MASK) as usize
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
        unsafe { &mut *self.header_mut() }.count -= 1;

        if self.header().count == 0 {
            unsafe { dealloc(self.as_ptr(), Layout::new::<Header>()) }
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
        assert_eq!(Array::new(0).get(0.0.into()), NIL);
        assert_eq!(Array::new(0).get(1.0.into()), NIL);
        assert_eq!(Array::new(1).get(0.0.into()), NIL);
    }
}
