#[repr(u8)]
pub enum Instruction {
    Float64,
    Add,
    Subtract,
    Multiply,
    Divide,
    Lambda,
}
