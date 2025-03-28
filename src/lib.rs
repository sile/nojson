pub mod formatter;
//pub mod value2;
mod value;
pub mod value3;

pub use value::{FiniteF64, Value};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

// TODO: Display, FromStr
