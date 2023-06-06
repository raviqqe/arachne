#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod array;
mod float64;
mod symbol;
mod r#type;
mod value;

pub use array::Array;
pub use float64::Float64;
pub use value::{Value, NIL};
