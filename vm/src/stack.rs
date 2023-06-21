use std::{
    alloc::{alloc, Layout},
    ptr::{copy, read, write},
};

#[derive(Debug)]
pub struct Stack<T> {
    base: *mut T,
    ptr: *mut T,
    capacity: usize,
}

impl<T> Stack<T> {
    #[inline(always)]
    pub fn new(capacity: usize) -> Self {
        let ptr = unsafe { alloc(Layout::array::<T>(capacity).unwrap()) } as *mut T;

        Self {
            base: ptr,
            ptr,
            capacity,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        if self.len() >= self.capacity {
            panic!("stack overflow");
        }

        unsafe {
            write(self.ptr, value);
        }

        self.ptr = unsafe { self.ptr.add(1) };
    }

    #[inline(always)]
    pub fn pop(&mut self) -> T {
        self.ptr = unsafe { self.ptr.sub(1) };

        unsafe { read(self.ptr) }
    }

    #[inline(always)]
    pub fn peek(&self, index: usize) -> &T {
        unsafe { &*self.ptr.sub(index + 1) }
    }

    #[inline(always)]
    pub fn top(&self) -> &T {
        self.peek(0)
    }

    #[inline(always)]
    pub fn truncate(&mut self, start: usize, end: usize) {
        for index in start..end {
            unsafe { read(self.base.add(index)) };
        }

        let count = self.len() - end;

        unsafe {
            copy(self.base.add(end), self.base.add(start), count);

            self.ptr = self.base.add(start).add(count);
        };
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        (unsafe { self.ptr.offset_from(self.base) }) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_stack<T>() -> Stack<T> {
        Stack::new(256)
    }

    #[test]
    fn push_and_pop_scalars() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    fn push_and_pop_containers() {
        let mut stack = create_stack();
        stack.push(Box::new(1));
        stack.push(Box::new(2));

        assert_eq!(stack.pop(), Box::new(2));
        assert_eq!(stack.pop(), Box::new(1));
    }

    #[test]
    fn peek() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.peek(0), &2);
        assert_eq!(stack.peek(1), &1);
    }

    #[test]
    fn truncate() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);

        stack.truncate(0, 1);

        assert_eq!(stack.pop(), 2);
    }

    #[test]
    fn truncate_overlapping() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        stack.truncate(0, 1);

        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.pop(), 2);
    }

    #[test]
    #[should_panic]
    fn overflow() {
        let mut stack = create_stack();

        loop {
            stack.push(0);
        }
    }
}
