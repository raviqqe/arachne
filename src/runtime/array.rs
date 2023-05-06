use super::Value;

pub struct Array(u64);

impl Array {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get(&self, index: Value) -> Value {
        todo!()
    }

    pub fn set(&self, index: Value, value: Value) -> Value {
        todo!()
    }

    pub fn len(&self) -> Value {
        todo!()
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}
