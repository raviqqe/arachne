use runtime::{Value, NIL};
use std::ptr::{copy, read, write};

#[derive(Debug)]
pub struct Stack {
    base: *mut Value,
    pointer: *mut Value,
}

impl Stack {
    pub fn new() -> Self {
        let values = Box::<[Value]>::leak(vec![NIL; 1 << 20].into());

        Self {
            base: &mut values[0],
            pointer: &mut values[0],
        }
    }

    pub fn push(&mut self, value: Value) {
        unsafe {
            write(self.pointer, value);
        }

        self.pointer = unsafe { self.pointer.add(1) };
    }

    pub fn pop(&mut self) -> Value {
        self.pointer = unsafe { self.pointer.sub(1) };

        unsafe { read(self.pointer) }
    }

    pub fn peek(&self, index: usize) -> &Value {
        unsafe { &*self.pointer.sub(index + 1) }
    }

    pub fn truncate(&mut self, start: usize, end: usize) {
        for index in start..end {
            unsafe { read(self.base.add(index)) };
        }

        let count = self.len() - end;
        unsafe {
            copy(self.base.add(end), self.base.add(start), count);

            self.pointer = self.base.add(start).add(count);
        };
    }

    pub fn len(&self) -> usize {
        (unsafe { self.pointer.offset_from(self.base) }) as usize
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
