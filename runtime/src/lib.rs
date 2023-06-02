#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod array;
mod number;
mod r#type;
mod value;

pub use array::Array;
pub use number::Float64;
pub use r#type::Type;
pub use value::{Value, NIL};
