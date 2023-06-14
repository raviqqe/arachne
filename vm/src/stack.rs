use runtime::Value;

#[derive(Debug)]
pub struct Stack {
    values: Vec<u64>,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Self {
            values: Vec::with_capacity(size),
        }
    }

    pub fn peek(&self, index: usize) -> &Value {
        unsafe { &*(self.values.get(index).unwrap() as *const u64).cast::<Value>() }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value.into_raw());
    }

    pub fn pop(&mut self) -> Value {
        unsafe { Value::from_raw(self.values.pop().expect("stack value")) }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
