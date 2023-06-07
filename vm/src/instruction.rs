#[repr(u8)]
pub enum Instruction {
    Null,
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
