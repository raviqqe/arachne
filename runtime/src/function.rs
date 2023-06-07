use crate::Value;
use alloc::alloc::alloc;
use core::alloc::Layout;

pub type FunctionId = u32;

pub struct Function {
    ptr: *const u8,
}

pub struct FunctionHeader {
    id: FunctionId,
}

impl Function {
    pub fn new(id: FunctionId, environment: &[Value]) -> Self {
        let (layout, _) = Layout::new::<FunctionHeader>()
            .extend(Layout::array::<Value>(environment.len()).unwrap())
            .unwrap();
        let ptr = unsafe { alloc(layout) };

        Self { ptr }
    }
}
