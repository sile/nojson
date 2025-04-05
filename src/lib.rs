//! TODO
//! - One-to-one mapping between Rust types and JSON texts is not needed.
//!   - You can gain the merits of both type-level probgramming and flexibility of imperative code.
//! - Rather toolbox than a monilitic framework.
//! - Easy to add custom validtions:
//!   - Application specific validation error can be associated with the errorneous JSON value position at the JSON text.
#![warn(missing_docs)]

mod display_json;
mod format;
mod from_raw_json_value;
mod kind;
mod parse;
mod parse_error;
mod raw;

use std::{fmt::Display, str::FromStr};

pub use display_json::DisplayJson;
pub use format::{JsonArrayFormatter, JsonFormatter, JsonObjectFormatter};
pub use from_raw_json_value::FromRawJsonValue;
pub use kind::JsonValueKind;
pub use raw::{JsonParseError, RawJson, RawJsonValue};

/// Marker struct that makes it possible to execute JSON parsing and generation via [`FromStr`] and [`Display`] traits.
///
/// This is a handy way to operating on JSON, but if you need more control,
/// please consider to use [`RawJson`] (for JSON pasing) and [`json()`] (for JSON generation) instead.
///
/// # Examples
///
/// Parse a JSON text:
/// ```
/// use nojson::Json;
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// // As `[Option<u32>; 3]` type implements the `FromRawJsonValue` trait,
/// // you can use the `std::str::parse()` method to parse JSON by wrapping the type by `Json`.
/// let text = "[1, null, 2]";
/// let value: Json(_) = text.parse()?;
/// assert_eq!(value, [Some(1), None, Some(2)]);
/// # OK(())
/// # }
/// ```
///
/// Generate a JSON type from a Rust type:
/// ```
/// use nojson::Json;
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// // As `[Option<u32>; 3]` type implements the `DisplyJson` trait too,
/// // you can use the `std::fmt::Display::to_string()` method to generate JSON by wrapping the type by `Json`.
/// let value = [Some(1), None, Some(2)];
/// assert_eq!(Json(value).to_string(), "[1,null,2]");
/// # OK(())
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json<T>(#[allow(missing_docs)] pub T);

impl<T: DisplayJson> Display for Json<T> {
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

pub fn json<F>(f: F) -> impl DisplayJson + Display
where
    F: Fn(&mut JsonFormatter<'_, '_>) -> std::fmt::Result,
{
    InplaceJson(f)
}

struct InplaceJson<F>(F);

impl<F> Display for InplaceJson<F>
where
    F: Fn(&mut JsonFormatter<'_, '_>) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Json(self))
    }
}

impl<F> DisplayJson for InplaceJson<F>
where
    F: Fn(&mut JsonFormatter<'_, '_>) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        self.0(f)
    }
}
