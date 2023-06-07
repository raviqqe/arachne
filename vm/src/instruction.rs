#[repr(u8)]
pub enum Instruction {
    Nil,
    Constant,
    Call,
    Float64Add,
    Float64Subtract,
    Float64Multiply,
    Float64Divide,
    Lambda,
}

impl Instruction {
    pub fn size(&self) -> usize {
        (match self {
            Self::Constant => 4,
            Self::Lambda => todo!(),
            Self::Nil
            | Self::Call
            | Self::Float64Add
            | Self::Float64Subtract
            | Self::Float64Multiply
            | Self::Float64Divide => 0,
        }) + 1
    }
}
