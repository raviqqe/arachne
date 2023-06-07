use crate::Value;
use alloc::alloc::alloc;
use core::{alloc::Layout, marker::PhantomData};

pub type FunctionId = u32;

pub struct Function {
    ptr: *const u8,
}

pub struct FunctionHeader {
    id: FunctionId,
    environment: PhantomData<[Value]>,
}

impl Function {
    pub fn new(id: FunctionId, environment: &[Value]) -> Self {
        let (layout, _) = Layout::new::<FunctionHeader>()
            .extend(Layout::array::<Value>(environment.len()).unwrap())
            .unwrap();
        let ptr = unsafe { alloc(layout) };

        unsafe {
            *ptr.cast::<FunctionHeader>() = FunctionHeader {
                id,
                environment: Default::default(),
            };

            for (index, value) in environment.iter().enumerate() {
                *ptr.cast::<FunctionHeader>()
                    .add(1)
                    .cast::<Value>()
                    .add(index) = value.clone();
            }
        }

        Self { ptr }
    }
}
