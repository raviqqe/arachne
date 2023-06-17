use runtime::Value;

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    #[inline]
    pub fn new(size: usize) -> Self {
        Self {
            values: Vec::with_capacity(size),
        }
    }

    #[inline]
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Value {
        self.values.pop().expect("stack value")
    }

    #[inline]
    pub fn peek(&self, index: usize) -> &Value {
        self.values.get(self.get_index(index)).unwrap()
    }

    #[inline]
    pub fn truncate(&mut self, start: usize, end: usize) {
        self.values.splice(start..end, []);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    fn get_index(&self, index: usize) -> usize {
        self.values.len() - 1 - index
    }
}
