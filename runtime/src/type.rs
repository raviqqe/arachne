#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Array,
    Float64,
    Integer32,
    Closure,
    Symbol,
}
