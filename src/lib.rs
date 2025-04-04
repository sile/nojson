mod format;
mod from_raw_json_value;
mod kind;
mod parse;
mod parse_error;
mod raw;

use std::str::FromStr;

pub use format::{DisplayJson, JsonArrayFormatter, JsonFormatter, JsonObjectFormatter, json};
pub use from_raw_json_value::FromRawJsonValue;
pub use kind::JsonValueKind;
pub use raw::{JsonParseError, RawJson, RawJsonValue};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(pub T);

impl<T: DisplayJson> std::fmt::Display for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = JsonFormatter::new(f);
        self.0.fmt(&mut fmt)?;
        Ok(())
    }
}

impl<T> FromStr for Json<T>
where
    T: for<'a> FromRawJsonValue<'a>,
{
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = RawJson::parse(s)?;
        T::from_raw_json_value(json.value()).map(Self)
    }
}
