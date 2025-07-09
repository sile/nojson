//! A flexible Rust JSON library with no dependencies and no macros.
//!
//! `nojson` is a flexible and ergonomic JSON library for Rust that offers a balance between the type-safety of Rust and the dynamic nature of JSON.
//! Unlike [`serde`](https://crates.io/crates/serde), which typically requires one-to-one mapping between Rust types and JSON structures (or other serialization formats),
//! `nojson` provides a toolbox approach that allows you to leverage both type-level programming and imperative code flexibility.
//!
//! ## Features
//!
//! - **No strict one-to-one type mapping required** - Mix type-level programming with imperative flexibility as needed
//! - **Clean parsing error messages** with position information for better debugging
//! - **Customizable validation** - Add application-specific validation rules with proper error context
//! - **Flexible formatting options** including pretty-printing with customizable indentation
//! - **Low-level access** to the JSON structure when needed
//! - **High-level conveniences** for common JSON operations
//!
//! ## Core Design Principles
//!
//! - A toolbox rather than a monolithic framework
//! - Gain the benefits of both type-level programming and imperative code
//! - Easy to add custom validations with rich error context
//! - Error messages that precisely indicate the problematic position in the JSON text
//!
//! ## Getting Started
//!
//! ### Parsing JSON with Strong Typing
//!
//! The [`Json<T>`] wrapper allows parsing JSON text into Rust types that implement `TryFrom<RawJsonValue<'_, '_>>`:
//!
//! ```
//! use nojson::Json;
//!
//! fn main() -> Result<(), nojson::JsonParseError> {
//!     // Parse a JSON array into a typed Rust array
//!     let text = "[1, null, 2]";
//!     let value: Json<[Option<u32>; 3]> = text.parse()?;
//!     assert_eq!(value.0, [Some(1), None, Some(2)]);
//!     Ok(())
//! }
//! ```
//!
//! ### Generating JSON
//!
//! The [`DisplayJson`] trait allows converting Rust types to JSON:
//!
//! ```
//! use nojson::Json;
//!
//! // Generate a JSON array from a Rust array
//! let value = [Some(1), None, Some(2)];
//! assert_eq!(Json(value).to_string(), "[1,null,2]");
//! ```
//!
//! ### In-place JSON Generation with Formatting
//!
//! The [`json()`] function provides a convenient way to generate JSON with custom formatting:
//!
//! ```
//! use nojson::json;
//!
//! // Compact JSON
//! let compact = json(|f| f.value([1, 2, 3]));
//! assert_eq!(compact.to_string(), "[1,2,3]");
//!
//! // Pretty-printed JSON with custom indentation
//! let pretty = json(|f| {
//!     f.set_indent_size(2);
//!     f.set_spacing(true);
//!     f.array(|f| {
//!         f.element(1)?;
//!         f.element(2)?;
//!         f.element(3)
//!     })
//! });
//!
//! assert_eq!(
//!     format!("\n{}", pretty),
//!     r#"
//! [
//!   1,
//!   2,
//!   3
//! ]"#
//! );
//! ```
//!
//! ### Custom Types
//!
//! Implementing [`DisplayJson`] and `TryFrom<RawJsonValue<'_, '_>>` for your own types:
//!
//! ```
//! use nojson::{DisplayJson, Json, JsonFormatter, JsonParseError, RawJsonValue};
//!
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! impl DisplayJson for Person {
//!     fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
//!         f.object(|f| {
//!             f.member("name", &self.name)?;
//!             f.member("age", self.age)
//!         })
//!     }
//! }
//!
//! impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for Person {
//!     type Error = JsonParseError;
//!
//!     fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
//!         let ([name, age], []) = value.to_fixed_object(["name", "age"], [])?;
//!         Ok(Person {
//!             name: name.try_into()?,
//!             age: age.try_into()?,
//!         })
//!     }
//! }
//!
//! fn main() -> Result<(), JsonParseError> {
//!     // Parse JSON to Person
//!     let json_text = r#"{"name":"Alice","age":30}"#;
//!     let person: Json<Person> = json_text.parse()?;
//!
//!     // Generate JSON from Person
//!     assert_eq!(Json(&person.0).to_string(), json_text);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Features
//!
//! ### Custom Validations
//!
//! You can add custom validations using [`RawJsonValue::invalid()`]:
//!
//! ```
//! use nojson::{JsonParseError, RawJson, RawJsonValue};
//!
//! fn parse_positive_number(text: &str) -> Result<u32, JsonParseError> {
//!     let json = RawJson::parse(text)?;
//!     let raw_value = json.value();
//!
//!     let num: u32 = raw_value.as_number_str()?
//!         .parse()
//!         .map_err(|e| raw_value.invalid(e))?;
//!
//!     if num == 0 {
//!         return Err(raw_value.invalid("Expected a positive number, got 0"));
//!     }
//!
//!     Ok(num)
//! }
//! ```
//!
//! ### Error Handling with Context
//!
//! Rich error information helps with debugging:
//!
//! ```
//! use nojson::{JsonParseError, RawJson};
//!
//! let text = r#"{"invalid": 123e++}"#;
//! let result = RawJson::parse(text);
//!
//! if let Err(error) = result {
//!     println!("Error: {}", error);
//!
//!     // Get line and column information
//!     if let Some((line, column)) = error.get_line_and_column_numbers(text) {
//!         println!("At line {}, column {}", line, column);
//!     }
//!
//!     // Get the full line with the error
//!     if let Some(line_text) = error.get_line(text) {
//!         println!("Line content: {}", line_text);
//!     }
//! }
//! ```
#![warn(missing_docs)]

mod display_json;
mod format;
mod kind;
mod parse;
mod parse_error;
mod raw;
mod try_from_impls;

use std::{fmt::Display, str::FromStr};

pub use display_json::DisplayJson;
pub use format::{JsonArrayFormatter, JsonFormatter, JsonObjectFormatter};
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
/// // Since the `[Option<u32>; 3]` type implements `TryFrom<RawJsonValue<'_, '_>>`,
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
/// // Since the `[Option<u32>; 3]` type also implements the `DisplayJson` trait,
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
    T: for<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = RawJson::parse(s)?;
        raw.value().try_into().map(Self)
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
