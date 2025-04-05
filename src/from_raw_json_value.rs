use std::str::FromStr;

use crate::{JsonParseError, RawJsonValue};

/// Converts a raw JSON value to a specific Rust type.
///
/// This trait allows for extracting typed values from untyped [`RawJsonValue`]
/// representations, performing necessary type checking and conversions.
///
/// Implementing this trait enables a type to be deserialized from JSON data.
/// Once a type implements [`FromRawJsonValue`], you can use [`Json`][crate::Json] to parse
/// JSON text to that type through Rust's standard [`FromStr`] trait.
///
/// # Examples
///
/// Parse a JSON array to a vector of integers:
///
/// ```
/// use nojson::{Json, RawJson, JsonParseError};
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// // Parse a JSON text via `str::parse()`.
/// let numbers: Json<[u32; 3]> = "[1, 2, 3]".parse()?;
/// assert_eq!(numbers.0, [1, 2, 3]);
///
/// // Alternatively, you can use `RawJson::parse()`,
/// // which offers a more flexible approach for converting JSON values and
/// // generating context-rich error messages.
/// let raw = RawJson::parse("[1, 2, 3]")?;
///
/// // For types that implement the `FromRawJsonValue` trait,
/// // the `RawJsonValue::try_to()` method can be used for conversion.
/// let numbers: [u32; 3] = raw.value().try_to()?;
/// assert_eq!(numbers, [1, 2, 3]);
/// # Ok(())
/// # }
/// ```
///
/// Parse a JSON object to a custom struct (requires implementing [`FromRawJsonValue`] for your struct):
///
/// ```
/// use nojson::{Json, RawJsonValue, JsonParseError, FromRawJsonValue};
///
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// impl<'text> FromRawJsonValue<'text> for Person {
///     fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
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
///
/// Parse a rational number represented as a JSON string:
///
/// ```
/// use nojson::{Json, RawJsonValue, JsonParseError, FromRawJsonValue};
/// use std::str::FromStr;
///
/// #[derive(Debug, PartialEq)]
/// struct Rational {
///     numerator: i32,
///     denominator: i32,
/// }
///
/// impl<'text> FromRawJsonValue<'text> for Rational {
///     fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
///         // Get the raw string content of the JSON value.
///         let fraction_str = value.to_unquoted_string_str()?;
///
///         // Split by the '/' character and parse components.
///         let parts: Vec<&str> = fraction_str.split('/').collect();
///         if parts.len() != 2 {
///             return Err(JsonParseError::invalid_value(value, "Expected format 'numerator/denominator'"));
///         }
///
///         let numerator = parts[0].parse()
///             .map_err(|_| JsonParseError::invalid_value(value, "Invalid numerator"))?;
///         let denominator = parts[1].parse()
///             .map_err(|_| JsonParseError::invalid_value(value, "Invalid denominator"))?;
///
///         if denominator == 0 {
///             return Err(JsonParseError::invalid_value(value, "Denominator cannot be zero"));
///         }
///
///         Ok(Rational { numerator, denominator })
///     }
/// }
///
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// let fraction: Json<Rational> = r#""3/4""#.parse()?;
/// assert_eq!(fraction.0, Rational { numerator: 3, denominator: 4 });
/// # Ok(())
/// # }
/// ```
pub trait FromRawJsonValue<'text>: Sized {
    /// Attempts to convert a raw JSON value to this type.
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError>;
}

impl<'text, T> FromRawJsonValue<'text> for Box<T>
where
    T: FromRawJsonValue<'text>,
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        T::from_raw_json_value(value).map(Box::new)
    }
}

impl<'text, T: FromRawJsonValue<'text>> FromRawJsonValue<'text> for Option<T> {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        if value.kind().is_null() {
            Ok(None)
        } else {
            value.try_to().map(Some)
        }
    }
}

impl<'text> FromRawJsonValue<'text> for bool {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value
            .as_bool_str()?
            .parse()
            .map_err(|e| JsonParseError::invalid_value(value, e))
    }
}

fn parse_integer<T>(value: RawJsonValue<'_, '_>) -> Result<T, JsonParseError>
where
    T: FromStr,
    T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
{
    value
        .as_integer_str()?
        .parse()
        .map_err(|e| JsonParseError::invalid_value(value, e))
}

impl<'text> FromRawJsonValue<'text> for i8 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for u8 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for i16 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for u16 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for i32 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for u32 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for i64 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for u64 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for i128 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for u128 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for isize {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for usize {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroI8 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroU8 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroI16 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroU16 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroI32 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroU32 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroI64 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroU64 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroI128 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroU128 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroIsize {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

impl<'text> FromRawJsonValue<'text> for std::num::NonZeroUsize {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_integer(value)
    }
}

fn parse_float<T>(value: RawJsonValue<'_, '_>) -> Result<T, JsonParseError>
where
    T: FromStr,
    T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
{
    value
        .as_number_str()?
        .parse()
        .map_err(|e| JsonParseError::invalid_value(value, e))
}

impl<'text> FromRawJsonValue<'text> for f32 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_float(value)
    }
}

impl<'text> FromRawJsonValue<'text> for f64 {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        parse_float(value)
    }
}

impl<'text> FromRawJsonValue<'text> for String {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| JsonParseError::invalid_value(value, e))
    }
}

impl<'text> FromRawJsonValue<'text> for std::borrow::Cow<'text, str> {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value.to_unquoted_string_str()
    }
}

impl<'text, T: FromRawJsonValue<'text>> FromRawJsonValue<'text> for Vec<T> {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value.to_array()?.map(T::from_raw_json_value).collect()
    }
}

impl<'text, T: FromRawJsonValue<'text>> FromRawJsonValue<'text> for std::collections::VecDeque<T> {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value.to_array()?.map(T::from_raw_json_value).collect()
    }
}

impl<'text, T> FromRawJsonValue<'text> for std::collections::BTreeSet<T>
where
    T: FromRawJsonValue<'text> + Ord,
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value.to_array()?.map(T::from_raw_json_value).collect()
    }
}

impl<'text, T> FromRawJsonValue<'text> for std::collections::HashSet<T>
where
    T: FromRawJsonValue<'text> + Eq + std::hash::Hash,
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value.to_array()?.map(T::from_raw_json_value).collect()
    }
}

impl<'text, T: FromRawJsonValue<'text>, const N: usize> FromRawJsonValue<'text> for [T; N] {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let values = value.to_fixed_array::<N>()?;
        let mut results = values.map(|v| v.try_to().map_err(Some));
        for result in &mut results {
            if let Err(e) = result {
                return Err(e.take().expect("infallible"));
            }
        }
        Ok(results.map(|r| r.expect("infallible")))
    }
}

impl<'text> FromRawJsonValue<'text> for () {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [] = value.to_fixed_array()?;
        Ok(())
    }
}

impl<'text, T0: FromRawJsonValue<'text>> FromRawJsonValue<'text> for (T0,) {
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0] = value.to_fixed_array()?;
        Ok((v0.try_to()?,))
    }
}

impl<'text, T0: FromRawJsonValue<'text>, T1: FromRawJsonValue<'text>> FromRawJsonValue<'text>
    for (T0, T1)
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0, v1] = value.to_fixed_array()?;
        Ok((v0.try_to()?, v1.try_to()?))
    }
}

impl<'text, T0: FromRawJsonValue<'text>, T1: FromRawJsonValue<'text>, T2: FromRawJsonValue<'text>>
    FromRawJsonValue<'text> for (T0, T1, T2)
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0, v1, v2] = value.to_fixed_array()?;
        Ok((v0.try_to()?, v1.try_to()?, v2.try_to()?))
    }
}

impl<
    'text,
    T0: FromRawJsonValue<'text>,
    T1: FromRawJsonValue<'text>,
    T2: FromRawJsonValue<'text>,
    T3: FromRawJsonValue<'text>,
> FromRawJsonValue<'text> for (T0, T1, T2, T3)
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0, v1, v2, v3] = value.to_fixed_array()?;
        Ok((v0.try_to()?, v1.try_to()?, v2.try_to()?, v3.try_to()?))
    }
}

impl<
    'text,
    T0: FromRawJsonValue<'text>,
    T1: FromRawJsonValue<'text>,
    T2: FromRawJsonValue<'text>,
    T3: FromRawJsonValue<'text>,
    T4: FromRawJsonValue<'text>,
> FromRawJsonValue<'text> for (T0, T1, T2, T3, T4)
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0, v1, v2, v3, v4] = value.to_fixed_array()?;
        Ok((
            v0.try_to()?,
            v1.try_to()?,
            v2.try_to()?,
            v3.try_to()?,
            v4.try_to()?,
        ))
    }
}

impl<
    'text,
    T0: FromRawJsonValue<'text>,
    T1: FromRawJsonValue<'text>,
    T2: FromRawJsonValue<'text>,
    T3: FromRawJsonValue<'text>,
    T4: FromRawJsonValue<'text>,
    T5: FromRawJsonValue<'text>,
> FromRawJsonValue<'text> for (T0, T1, T2, T3, T4, T5)
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0, v1, v2, v3, v4, v5] = value.to_fixed_array()?;
        Ok((
            v0.try_to()?,
            v1.try_to()?,
            v2.try_to()?,
            v3.try_to()?,
            v4.try_to()?,
            v5.try_to()?,
        ))
    }
}

impl<
    'text,
    T0: FromRawJsonValue<'text>,
    T1: FromRawJsonValue<'text>,
    T2: FromRawJsonValue<'text>,
    T3: FromRawJsonValue<'text>,
    T4: FromRawJsonValue<'text>,
    T5: FromRawJsonValue<'text>,
    T6: FromRawJsonValue<'text>,
> FromRawJsonValue<'text> for (T0, T1, T2, T3, T4, T5, T6)
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0, v1, v2, v3, v4, v5, v6] = value.to_fixed_array()?;
        Ok((
            v0.try_to()?,
            v1.try_to()?,
            v2.try_to()?,
            v3.try_to()?,
            v4.try_to()?,
            v5.try_to()?,
            v6.try_to()?,
        ))
    }
}

impl<
    'text,
    T0: FromRawJsonValue<'text>,
    T1: FromRawJsonValue<'text>,
    T2: FromRawJsonValue<'text>,
    T3: FromRawJsonValue<'text>,
    T4: FromRawJsonValue<'text>,
    T5: FromRawJsonValue<'text>,
    T6: FromRawJsonValue<'text>,
    T7: FromRawJsonValue<'text>,
> FromRawJsonValue<'text> for (T0, T1, T2, T3, T4, T5, T6, T7)
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        let [v0, v1, v2, v3, v4, v5, v6, v7] = value.to_fixed_array()?;
        Ok((
            v0.try_to()?,
            v1.try_to()?,
            v2.try_to()?,
            v3.try_to()?,
            v4.try_to()?,
            v5.try_to()?,
            v6.try_to()?,
            v7.try_to()?,
        ))
    }
}

impl<'text, K, V> FromRawJsonValue<'text> for std::collections::BTreeMap<K, V>
where
    K: FromStr + Ord,
    K::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    V: FromRawJsonValue<'text>,
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value
            .to_object()?
            .map(|(k, v)| {
                Ok((
                    k.to_unquoted_string_str()?
                        .parse()
                        .map_err(|e| JsonParseError::invalid_value(k, e))?,
                    v.try_to()?,
                ))
            })
            .collect()
    }
}

impl<'text, K, V> FromRawJsonValue<'text> for std::collections::HashMap<K, V>
where
    K: FromStr + Eq + std::hash::Hash,
    K::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    V: FromRawJsonValue<'text>,
{
    fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
        value
            .to_object()?
            .map(|(k, v)| {
                Ok((
                    k.to_unquoted_string_str()?
                        .parse()
                        .map_err(|e| JsonParseError::invalid_value(k, e))?,
                    v.try_to()?,
                ))
            })
            .collect()
    }
}
