use super::{
    value::{ARRAY_MASK, NIL},
    Number, Value,
};
use std::{
    alloc::{alloc, Layout},
    mem::{align_of, size_of},
};

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

        Self(unsafe { alloc(layout) } as usize as u64 | ARRAY_MASK)
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

    pub fn set(&self, index: Value, value: Value) -> Value {
        todo!()
    }

    pub fn len(&self) -> Value {
        Number::from(self.len_usize() as f64).into()
    }

    fn len_usize(&self) -> usize {
        self.header().length
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    fn header(&self) -> &Header {
        let ptr = self.as_ptr() as *const Header;

        unsafe { &*ptr }
    }

    fn element_ptr(&self) -> *mut u8 {
        (self.0 as usize + Layout::new::<Header>().size()) as *mut u8
    }

    fn as_ptr(&self) -> *mut u8 {
        self.0 as *mut u8
    }
}
