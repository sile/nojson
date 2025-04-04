use crate::{JsonParseError, RawJsonValue};

/// Converts a raw JSON value into a specific Rust type.
///
/// This trait allows for extracting typed values from untyped [`RawJsonValue`]
/// representations, performing necessary type checking and conversions.
///
/// Implementing this trait enables a type to be deserialized from JSON data.
/// Once a type implements [`FromRawJsonValue`], you can use [`Json`][crate::Json] to parse
/// JSON text into that type through Rust's standard [`FromStr`][std::str::FromStr] trait.
///
/// # Examples
///
/// Parse a JSON array into a vector of integers:
///
/// ```
/// use nojson::Json;
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// let numbers: Json<Vec<u32>> = "[1, 2, 3]".parse()?;
/// assert_eq!(numbers.0, [1, 2, 3]);
/// # Ok(())
/// # }
/// ```
///
/// Parse a JSON object into a custom struct (requires implementing [`FromRawJsonValue`] for your struct):
///
/// ```
/// use nojson::{Json, RawJsonValue, JsonParseError, FromRawJsonValue};
///
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// impl<'a> FromRawJsonValue<'a> for Person {
///     fn from_raw_json_value(value: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
///         let ([name, age], []) = value.to_fixed_object(["name","age"],[])?;
///         Ok(Person {
///             name: name.try_to()?,
///             age: age.try_to()?,
///         })
///     }
/// }
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// let person: Json<Person> = r#"{"name":"Alice","age":30}"#.parse()?;
/// assert_eq!(person.0.name, "Alice");
/// assert_eq!(person.0.age, 30);
/// # Ok(())
/// # }
/// ```
pub trait FromRawJsonValue<'a>: Sized {
    /// Attempts to convert a raw JSON value into this type.
    fn from_raw_json_value(value: RawJsonValue<'a>) -> Result<Self, JsonParseError>;
}

impl<'a, T: FromRawJsonValue<'a>> FromRawJsonValue<'a> for Box<T> {
    fn from_raw_json_value(value: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        T::from_raw_json_value(value).map(Box::new)
    }
}

// TODO: impl FromRawJsonValue for i8,u8,i16,u16,i32,i64,u64,i128,u128,isize,usize, and NonZero types
impl<'a> FromRawJsonValue<'a> for u32 {
    fn from_raw_json_value(value: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        value.as_integer()?.parse()
    }
}

impl<'a> FromRawJsonValue<'a> for String {
    fn from_raw_json_value(value: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        value.as_string()?.parse()
    }
}

impl<'a, T: FromRawJsonValue<'a>> FromRawJsonValue<'a> for Vec<T> {
    fn from_raw_json_value(value: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        value
            .to_array_values()?
            .map(T::from_raw_json_value)
            .collect()
    }
}

// TODO: Add impl for Cow<'a, str>, String, u8, i8, f32, f64, Option<T>, Vec<T>, [T; N], HashMap etc
