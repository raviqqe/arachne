extern crate alloc;

mod array;
mod closure;
mod float64;
mod integer32;
mod symbol;
mod r#type;
mod typed_value;
mod value;
mod vm;

pub use array::Array;
pub use closure::{Closure, ClosureId};
pub use float64::Float64;
pub use r#type::Type;
pub use symbol::Symbol;
pub use typed_value::{TypedValue, TypedValueRef};
pub use value::{Value, NIL};
pub use vm::*;
