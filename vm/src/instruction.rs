#[repr(u8)]
pub enum Instruction {
    Nil,
    Call,
    Symbol,
    Float64,
    Float64Add,
    Float64Subtract,
    Float64Multiply,
    Float64Divide,
    Lambda,
}
