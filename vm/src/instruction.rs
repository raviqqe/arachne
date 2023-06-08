#[repr(u8)]
#[derive(Clone, Copy, Debug, num_derive::FromPrimitive)]
pub enum Instruction {
    Null,
    Nil,
    Constant,
    Local, // TODO Rename to `Variable`?
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
    Drop,
    Dump,
}
