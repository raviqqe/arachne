use runtime::Value;

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    #[inline(always)]
    pub fn new(size: usize) -> Self {
        Self {
            values: Vec::with_capacity(size),
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Value {
        self.values.pop().expect("stack value")
    }

    #[inline(always)]
    pub fn peek(&self, index: usize) -> &Value {
        self.values.get(self.get_index(index)).unwrap()
    }

    #[inline(always)]
    pub fn truncate(&mut self, start: usize, end: usize) {
        self.values.splice(start..end, []);
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline(always)]
    fn get_index(&self, index: usize) -> usize {
        self.values.len() - 1 - index
    }
}
