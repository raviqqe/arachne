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

    #[inline]
    pub fn pop(&mut self) -> Value {
        self.values.pop().expect("stack value")
    }

    pub fn peek(&self, index: usize) -> &Value {
        self.values.get(self.get_index(index)).unwrap()
    }

    pub fn truncate(&mut self, start: usize, end: usize) {
        self.values.splice(start..end, []);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    fn get_index(&self, index: usize) -> usize {
        self.values.len() - 1 - index
    }
}
