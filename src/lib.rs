pub mod formatter;
//pub mod value2;
pub mod value3;

pub mod format;
mod value;

use std::fmt::Display;

pub use format::DisplayJson;
pub use value::{FiniteF64, Value};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

impl<T: DisplayJson> Display for Json<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// TODO: FromStr
