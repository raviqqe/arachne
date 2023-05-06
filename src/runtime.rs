pub type Value = f64;

#[repr(transparent)]
pub struct Number(Value);

#[repr(transparent)]
pub struct Array(f64);

impl Array {
    pub fn new() -> Self {
        Self(0.0)
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
}
