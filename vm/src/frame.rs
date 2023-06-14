pub struct Frame {
    return_address: usize,
    frame_pointer: usize,
}

impl Frame {
    pub fn new(return_address: usize, frame_pointer: usize) -> Self {
        Self {
            return_address,
            frame_pointer,
        }
    }

    pub fn return_address(&self) -> usize {
        self.return_address
    }

    pub fn frame_pointer(&self) -> usize {
        self.frame_pointer
    }
}
