pub mod fmt;
pub mod num;
mod parse_error;
mod parser;
pub mod str;
mod value;

use str::{JsonParseError, JsonTextStr, JsonValueStr};
pub use value::{JsonValue, JsonValueKind};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

impl<T: fmt::DisplayJson> std::fmt::Display for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> std::str::FromStr for Json<T>
where
    T: for<'a> TryFrom<JsonValueStr<'a>, Error = JsonParseError>,
{
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = JsonTextStr::parse(s)?;
        json.value().try_into().map(Self)
    }
}
