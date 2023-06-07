use crate::Value;
use alloc::alloc::alloc;
use core::{alloc::Layout, marker::PhantomData};

pub type ClosureId = u32;

pub struct Closure {
    ptr: *const u8,
}

pub struct ClosureHeader {
    id: ClosureId,
    environment: PhantomData<[Value]>,
}

impl Closure {
    pub fn new(id: ClosureId, environment: &[Value]) -> Self {
        let (layout, _) = Layout::new::<ClosureHeader>()
            .extend(Layout::array::<Value>(environment.len()).unwrap())
            .unwrap();
        let ptr = unsafe { alloc(layout) };

        unsafe {
            *ptr.cast::<ClosureHeader>() = ClosureHeader {
                id,
                environment: Default::default(),
            };

            for (index, value) in environment.iter().enumerate() {
                *ptr.cast::<ClosureHeader>()
                    .add(1)
                    .cast::<Value>()
                    .add(index) = value.clone();
            }
        }

        Self { ptr }
    }
}

#[cfg(test)]
mod foo {}
