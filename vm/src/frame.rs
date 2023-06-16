pub struct Frame {
    pointer: u32,
    return_address: u32,
}

impl Frame {
    // TODO Swap the argument order.
    pub fn new(return_address: u32, pointer: u32) -> Self {
        Self {
            pointer,
            return_address,
        }
    }

    pub fn pointer(&self) -> u32 {
        self.pointer
    }

    pub fn return_address(&self) -> u32 {
        self.return_address
    }
}
