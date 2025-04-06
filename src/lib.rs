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

/// A marker struct that enables JSON parsing and generation through the [`FromStr`] and [`Display`] traits.
///
/// This provides a convenient way to work with JSON, but if you need more fine-grained control,
/// consider using [`RawJson`] (for JSON parsing) and [`json()`] (for JSON generation) instead.
///
/// # Examples
///
/// Parsing JSON text:
/// ```
/// use nojson::Json;
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// // Since the `[Option<u32>; 3]` type implements the `FromRawJsonValue` trait,
/// // you can use the `std::str::parse()` method to parse JSON by wrapping the type with `Json`.
/// let text = "[1, null, 2]";
/// let value: Json<[Option<u32>; 3]> = text.parse()?;
/// assert_eq!(value.0, [Some(1), None, Some(2)]);
/// # Ok(())
/// # }
/// ```
///
/// Generating JSON from a Rust type:
/// ```
/// use nojson::Json;
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// // Since the `[Option<u32>; 3]` type also implements the `DisplyJson` trait,
/// // you can use the `std::fmt::Display::to_string()` method to
/// // generate JSON by wrapping the type with `Json`.
/// let value = [Some(1), None, Some(2)];
/// assert_eq!(Json(value).to_string(), "[1,null,2]");
/// # Ok(())
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

/// Similiar to [`Json`], but can be used for pretty-printing and in-place JSON generation purposes.
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use nojson::json;
///
/// // Standard JSON serialization (compact)
/// let compact = json(|f| f.value([1, 2, 3]));
/// assert_eq!(compact.to_string(), "[1,2,3]");
/// ```
///
/// ## Pretty printing with custom indentation
///
/// ```
/// use nojson::json;
///
/// // Pretty-printed JSON with 2-space indentation
/// let pretty = json(|f| {
///     f.set_indent_size(2);
///     f.set_spacing(true);
///     f.value([1, 2, 3])
/// });
///
/// assert_eq!(
///     format!("\n{}", pretty),
///     r#"
/// [
///   1,
///   2,
///   3
/// ]"#
/// );
/// ```
///
/// ## Mixing formatting styles
///
/// ```
/// use nojson::{json, DisplayJson};
///
/// // You can nest formatters with different settings
/// let mixed = json(|f| {
///     f.set_indent_size(2);
///     f.set_spacing(true);
///     f.value([
///         &vec![1] as &dyn DisplayJson,
///         &json(|f| {
///             f.set_indent_size(0);
///             f.value(vec![2, 3])
///         }),
///     ])
/// });
///
/// assert_eq!(
///     format!("\n{}", mixed),
///     r#"
/// [
///   [
///     1
///   ],
///   [2, 3]
/// ]"#
/// );
/// ```
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

// TODO: error context generating function
