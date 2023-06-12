use crate::{value::CLOSURE_MASK, Value};
use alloc::alloc::{alloc, dealloc};
use core::{
    alloc::Layout,
    fmt::{self, Display, Formatter},
    mem::forget,
    ptr::{drop_in_place, write},
};

pub type ClosureId = u32;

pub struct Closure(u64);

#[repr(C)]
struct Header {
    count: usize,
    // In the current implementation, function IDs are equivalent to function pointers in
    // bytecodes.
    id: ClosureId,
    arity: u8,
    environment_size: u8,
}

impl Closure {
    pub fn new(id: ClosureId, arity: u8, environment_size: u8) -> Self {
        let (layout, _) = Layout::new::<Header>()
            .extend(Layout::array::<Value>(environment_size as usize).unwrap())
            .unwrap();
        let this = Self(unsafe { alloc(layout) } as u64 | CLOSURE_MASK);

        unsafe {
            *this.header_mut() = Header {
                count: 0,
                id,
                arity,
                environment_size,
            };
        }

        this
    }

    pub fn arity(&self) -> u8 {
        self.header().arity
    }

    pub fn write_environment(&mut self, index: usize, value: Value) {
        assert!(index < self.header().environment_size as usize);

        unsafe { write(self.environment_mut(index), value) }
    }

    /// # Safety
    ///
    /// The pointer must be valid.
    pub unsafe fn from_raw(ptr: u64) -> Self {
        Self(ptr)
    }

    pub fn id(&self) -> ClosureId {
        self.header().id
    }

    pub fn is_nil(&self) -> bool {
        self.0 == 0
    }

    pub fn into_raw(self) -> u64 {
        let ptr = self.0;

        forget(self);

        ptr
    }

    fn header(&self) -> &Header {
        unsafe { &*self.header_mut() }
    }

    fn header_mut(&self) -> *mut Header {
        self.as_ptr() as *mut _
    }

    fn environment_mut(&self, index: usize) -> *mut Value {
        unsafe {
            self.as_ptr()
                .cast::<Header>()
                .add(1)
                .cast::<Value>()
                .add(index)
        }
    }

    fn as_ptr(&self) -> *mut u8 {
        (self.0 & !CLOSURE_MASK) as *mut _
    }
}

impl Clone for Closure {
    fn clone(&self) -> Self {
        if !self.is_nil() {
            unsafe { &mut *self.header_mut() }.count += 1;
        }

        Self(self.0)
    }
}

impl Drop for Closure {
    fn drop(&mut self) {
        if self.is_nil() {
        } else if self.header().count == 0 {
            unsafe {
                for index in 0..self.header().environment_size {
                    drop_in_place(self.environment_mut(index as usize));
                }

                dealloc(self.as_ptr(), Layout::new::<Header>());
            }
        } else {
            unsafe { &mut *self.header_mut() }.count -= 1;
        }
    }
}

impl Display for Closure {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "<closure {:x}>", self.0)
    }
}

impl TryFrom<Value> for Closure {
    type Error = Value;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_closure() {
            Ok(unsafe { Closure::from_raw(value.into_raw()) })
        } else {
            Err(value)
        }
    }
}

impl TryFrom<&Value> for &Closure {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if value.is_closure() {
            let ptr = value as *const _ as *const _;

            Ok(unsafe { &*ptr })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        Closure::new(0, 0, 0);
    }

    #[test]
    fn clone() {
        #[allow(clippy::redundant_clone)]
        let _ = Closure::new(0, 0, 0).clone();
    }

    #[test]
    fn clone_with_environment() {
        let value = [42.0.into()].into();
        let mut closure = Closure::new(0, 0, 1);

        closure.write_environment(0, value);

        #[allow(clippy::redundant_clone)]
        let _ = closure.clone();
    }
}
