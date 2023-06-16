#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variable {
    Bound(usize),
    Free(usize),
}
