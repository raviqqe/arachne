use runtime::Value;

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Self {
            values: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.values.pop().expect("stack value")
    }

    pub fn peek(&self, index: usize) -> &Value {
        self.values.get(self.values.len() - 1 - index).unwrap()
    }

    pub fn insert(&mut self, index: usize, value: Value) {
        self.values.insert(index, value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
