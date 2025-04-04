//!
//! - One-to-one mapping between Rust types and JSON texts is not needed.
//!   - You can gain the merits of both type-level probgramming and flexibility of imperative code.
//! - Rather toolbox than a monilitic framework.
//! - Easy to add custom validtions:
//!   - Application specific validation error can be associated with the errorneous JSON value position at the JSON text.
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
        let raw = RawJson::parse(s)?;
        raw.value().try_to().map(Self)
    }
}
