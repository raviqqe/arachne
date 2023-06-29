use runtime::Value;

#[derive(Debug)]
pub struct Prompt {
    tag: Value,
    frame_pointer: u32,
}

impl Prompt {
    pub fn new(tag: Value, frame_pointer: u32) -> Self {
        Self { tag, frame_pointer }
    }

    pub fn tag(&self) -> &Value {
        &self.tag
    }

    pub fn frame_pointer(&self) -> u32 {
        self.frame_pointer
    }
}
