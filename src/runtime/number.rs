#[repr(transparent)]
pub struct Number(f64);

impl Number {
    pub fn to_f64(&self) -> f64 {
        self.0
    }
}
