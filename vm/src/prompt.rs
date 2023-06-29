use runtime::Value;

#[derive(Debug)]
pub struct Prompt {
    tag: Value,
}

impl Prompt {
    pub fn new(tag: Value) -> Self {
        Self { tag }
    }
}
