use runtime::{Value, NIL};
use std::ptr::replace;

#[derive(Debug)]
pub struct Stack {
    values: Box<[Value]>,
    base: *mut Value,
    pointer: *mut Value,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        let mut values: Box<[Value]> = vec![NIL; size].into();

        Self {
            base: &mut values[0],
            pointer: &mut values[0],
            values,
        }
    }

    pub fn push(&mut self, value: Value) {
        unsafe {
            *self.pointer = value;
        }

        self.pointer = unsafe { self.pointer.add(1) };
    }

    pub fn pop(&mut self) -> Value {
        self.pointer = unsafe { self.pointer.sub(1) };

        unsafe { replace(self.pointer, NIL) }
    }

    pub fn peek(&self, index: usize) -> &Value {
        unsafe { &*self.pointer.sub(index + 1) }
    }

    pub fn truncate(&mut self, start: usize, end: usize) {
        todo!();
        // self.values.splice(start..end, []);
    }

    pub fn len(&self) -> usize {
        (unsafe { self.pointer.sub(self.base as usize) }) as usize
    }

    fn get_index(&self, index: usize) -> usize {
        self.values.len() - 1 - index
    }
}
