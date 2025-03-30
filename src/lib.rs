pub mod fmt;
pub mod num;
mod parser;
pub mod str;
mod value;

pub use value::{JsonValue, JsonValueKind};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

impl<T: fmt::DisplayJson> std::fmt::Display for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// impl<T: str::FromJsonStr> std::str::FromStr for Json<T> {
//     type Err = str::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let s = str::JsonStr::new(s)?;
//         T::from_json_str(&s).map(Self)
//     }
// }
