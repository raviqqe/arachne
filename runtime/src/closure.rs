use crate::{value::CLOSURE_MASK, Value};
use alloc::alloc::{alloc, dealloc};
use core::{
    alloc::Layout,
    fmt::{self, Display, Formatter},
    mem::forget,
    ptr::drop_in_place,
};

pub type ClosureId = u32;

pub struct Closure(u64);

#[repr(C)]
struct Header {
    count: usize,
    id: ClosureId,
    environment_size: u32,
}

impl Closure {
    pub fn new(id: ClosureId, environment: &[Value]) -> Self {
        let (layout, _) = Layout::new::<Header>()
            .extend(Layout::array::<Value>(environment.len()).unwrap())
            .unwrap();
        let ptr = unsafe { alloc(layout) };

        unsafe {
            *ptr.cast::<Header>() = Header {
                count: 0,
                id,
                environment_size: environment.len() as u32,
            };

            for (index, value) in environment.iter().enumerate() {
                *ptr.cast::<Header>().add(1).cast::<Value>().add(index) = value.clone();
            }
        }

        Self(ptr as u64 | CLOSURE_MASK)
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
                    drop_in_place(
                        self.as_ptr()
                            .cast::<Header>()
                            .add(1)
                            .cast::<Value>()
                            .add(index as usize),
                    );
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
        Closure::new(0, &[]);
    }

    #[test]
    fn clone() {
        #[allow(clippy::redundant_clone)]
        let _ = Closure::new(0, &[]).clone();
    }

    #[test]
    fn clone_with_environment() {
        #[allow(clippy::redundant_clone)]
        let _ = Closure::new(0, &[[42.0.into()].into()]).clone();
    }
}
