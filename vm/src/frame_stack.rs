use runtime::{Value, NIL};
use std::ptr::{read, write};

const SIZE: usize = 1 << 8;

#[derive(Debug)]
pub struct FrameStack {
    pointer: *mut Value,
}

impl FrameStack {
    pub fn new() -> Self {
        let values = Box::<[Value]>::leak(vec![NIL; SIZE].into());

        Self {
            pointer: &mut values[0],
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: Value) {
        if self.len() >= SIZE {
            panic!("stack overflow");
        }

        unsafe {
            write(self.pointer, value);
        }

        self.pointer = unsafe { self.pointer.add(1) };
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Value {
        self.pointer = unsafe { self.pointer.sub(1) };

        unsafe { read(self.pointer) }
    }

    #[inline(always)]
    pub fn peek(&self) -> &Value {
        unsafe { &*self.pointer.sub(index + 1) }
    }
}

impl Default for FrameStack {
    fn default() -> Self {
        Self::new()
    }
}
