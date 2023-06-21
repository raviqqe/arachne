use crate::frame::Frame;
use std::ptr::{read, write};

const SIZE: usize = 1 << 8;

#[derive(Debug)]
pub struct FrameStack {
    base: *mut Frame,
    pointer: *mut Frame,
}

impl FrameStack {
    pub fn new() -> Self {
        let values = Box::<[Frame]>::leak(vec![Frame::new(0, 0); SIZE].into());

        Self {
            base: &mut values[0],
            pointer: &mut values[0],
        }
    }

    #[inline(always)]
    pub fn push(&mut self, frame: Frame) {
        if self.len() >= SIZE {
            panic!("stack overflow");
        }

        unsafe {
            write(self.pointer, frame);
        }

        self.pointer = unsafe { self.pointer.add(1) };
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Frame {
        self.pointer = unsafe { self.pointer.sub(1) };

        unsafe { read(self.pointer) }
    }

    #[inline(always)]
    pub fn peek(&self) -> &Frame {
        unsafe { &*self.pointer.sub(1) }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        (unsafe { self.pointer.offset_from(self.base) }) as usize
    }
}

impl Default for FrameStack {
    fn default() -> Self {
        Self::new()
    }
}
