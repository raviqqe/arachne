use crate::{integer32::Integer32, Array, Closure, Float64, Symbol};

pub enum TypedValue {
    Array(Array),
    Float64(Float64),
    Integer32(Integer32),
    Closure(Closure),
    Symbol(Symbol),
}
