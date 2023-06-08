#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::ToPrimitive)]
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
    Equal,
    Array,
    Dump,
}
