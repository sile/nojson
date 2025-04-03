mod format;
mod kind;
mod parse;
mod parse_error;
mod raw;

pub use format::{DisplayJson, JsonArrayFormatter, JsonFormatter, JsonObjectFormatter, json};
pub use kind::JsonValueKind;
pub use raw::{FromRawJsonValue, JsonParseError, RawJson, RawJsonValue};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

impl<T: DisplayJson> std::fmt::Display for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = JsonFormatter::new(f);
        self.0.fmt(&mut fmt)?;
        Ok(())
    }
}

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
