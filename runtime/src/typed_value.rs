use crate::{integer32::Integer32, Array, Closure, Float64, Symbol};

pub enum TypedValue {
    Array(Array),
    Closure(Closure),
    Float64(Float64),
    Integer32(Integer32),
    Symbol(Symbol),
}

pub enum TypedValueRef<'a> {
    Array(&'a Array),
    Closure(&'a Closure),
    Float64(Float64),
    Integer32(Integer32),
    Symbol(Symbol),
}
