use crate::{Array, Closure, Float64, Symbol};

pub enum TypedValue {
    Nil,
    Array(Array),
    Float64(Float64),
    Closure(Closure),
    Symbol(Symbol),
}
