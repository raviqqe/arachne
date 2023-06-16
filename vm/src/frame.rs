pub struct Frame {
    return_address: u32,
    pointer: u32,
}

impl Frame {
    pub fn new(return_address: u32, frame_pointer: u32) -> Self {
        Self {
            return_address,
            pointer: frame_pointer,
        }
    }

    pub fn return_address(&self) -> u32 {
        self.return_address
    }

    pub fn pointer(&self) -> u32 {
        self.pointer
    }
}
