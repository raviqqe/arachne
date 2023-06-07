#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod array;
mod float64;
mod function;
mod symbol;
mod r#type;
mod value;

pub use array::Array;
pub use float64::Float64;
pub use function::{Function, FunctionId};
pub use r#type::Type;
pub use symbol::Symbol;
pub use value::{Value, NIL};
