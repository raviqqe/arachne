use super::Value;
use std::{
    alloc::{alloc, Layout},
    mem::size_of,
};

const ALIGNMENT: usize = 8;

pub struct Array(u64);

struct Header {
    count: usize,
    length: usize,
}

impl Array {
    pub fn new(size: usize) -> Self {
        let layout = Layout::new::<Header>()
            .extend(Layout::from_size_align(size_of::<usize>() * size, ALIGNMENT).unwrap())
            .unwrap()
            .0;

        Self(unsafe { alloc(layout) } as usize as u64 & (1 << 63))
    }

    pub fn get(&self, index: Value) -> Value {
        todo!()
    }

    pub fn set(&self, index: Value, value: Value) -> Value {
        todo!()
    }

    pub fn len(&self) -> Value {
        todo!()
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}
