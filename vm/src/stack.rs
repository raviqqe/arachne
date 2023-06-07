pub struct Stack {
    values: Vec<u64>,
}

impl Stack {
    pub fn push_f64(&mut self, value: f64) {
        self.values.push(value.to_bits());
    }

    pub fn pop_f64(&mut self) -> f64 {
        f64::from_bits(self.values.pop().unwrap())
    }
}
