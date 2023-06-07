#[repr(u8)]
pub enum Instruction {
    Nil,
    Constant,
    Get,
    Set,
    Length,
    Float64Add,
    Float64Subtract,
    Float64Multiply,
    Float64Divide,
    Call,
    Lambda,
}
