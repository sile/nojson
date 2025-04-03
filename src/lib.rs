pub mod builder;
mod fmt;
mod kind;
mod parse_error;
mod parser;
mod str; // TODO: rename

pub use fmt::{DisplayJson, JsonFormatter};
pub use kind::JsonValueKind;
pub use str::{FromRawJsonValue, JsonParseError, RawJson, RawJsonValue};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

// TODO
// impl<T: fmt::DisplayJson> std::fmt::Display for Json<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }

impl<T> std::str::FromStr for Json<T>
where
    T: for<'a> TryFrom<RawJsonValue<'a>, Error = JsonParseError>,
{
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = RawJson::parse(s)?;
        json.value().try_into().map(Self)
    }
}
