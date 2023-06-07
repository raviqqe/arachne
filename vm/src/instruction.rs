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
