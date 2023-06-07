use runtime::Value;

pub struct Stack {
    values: Vec<u64>,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Self {
            values: Vec::with_capacity(size),
        }
    }

    pub fn push_value(&mut self, value: Value) {
        self.values.push(value.to_raw());
    }

    pub fn pop_value(&mut self) -> Value {
        unsafe { Value::from_raw(self.values.pop().unwrap()) }
    }
}
