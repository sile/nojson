mod fmt;
mod kind;
mod parse_error;
mod parser;
mod str; // TODO: rename

pub use fmt::{DisplayJsonValue, JsonArrayFormatter, JsonObjectFormatter, JsonValueFormatter};
pub use kind::JsonValueKind;
pub use str::{FromRawJsonValue, JsonParseError, JsonText, RawJsonValue};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

impl<T: fmt::DisplayJsonValue> std::fmt::Display for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> std::str::FromStr for Json<T>
where
    T: for<'a> TryFrom<RawJsonValue<'a>, Error = JsonParseError>,
{
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = JsonText::parse(s)?;
        json.raw_value().try_into().map(Self)
    }
}
