pub mod formatter;
//pub mod value2;
pub mod value3;

mod format; // TDOO: pub fmt (?)
mod value;

use std::fmt::Display;

pub use format::{ArrayIter, DisplayJson, DisplayJsonString, ObjectIter};
pub use value::{FiniteF64, Value};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

impl<T: DisplayJson> Display for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// TODO: FromStr
