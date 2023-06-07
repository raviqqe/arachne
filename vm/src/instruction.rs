#[repr(u8)]
pub enum Instruction {
    Null,
    Nil,
    Constant,
    Local,
    Get,
    Set,
    Length,
    Add,
    Subtract,
    Multiply,
    Divide,
    Call,
    Lambda,
}
