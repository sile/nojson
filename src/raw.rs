use std::{borrow::Cow, ops::Range, str::FromStr};

#[cfg(doc)]
use crate::Json;
use crate::{parse::JsonParser, JsonValueKind};

pub use crate::parse_error::JsonParseError;

/// Converts a raw JSON value into a specific Rust type.
///
/// This trait allows for extracting typed values from untyped [`RawJsonValue`]
/// representations, performing necessary type checking and conversions.
///
/// Implementing this trait enables a type to be deserialized from JSON data.
/// Once a type implements [`FromRawJsonValue`], you can use [`Json`] to parse
/// JSON text into that type through Rust's standard [`FromStr`] trait.
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
///     fn from_raw_json_value(raw: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
///         let ([name, age], []) = raw.to_fixed_object(["name","age"],[])?;
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
    fn from_raw_json_value(raw: RawJsonValue<'a>) -> Result<Self, JsonParseError>;
}

impl<'a, T: FromRawJsonValue<'a>> FromRawJsonValue<'a> for Box<T> {
    fn from_raw_json_value(raw: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        T::from_raw_json_value(raw).map(Box::new)
    }
}

impl<'a> FromRawJsonValue<'a> for u32 {
    fn from_raw_json_value(raw: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        raw.as_integer()?.parse()
    }
}

impl<'a> FromRawJsonValue<'a> for String {
    fn from_raw_json_value(raw: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        raw.as_string()?.parse()
    }
}

impl<'a, T: FromRawJsonValue<'a>> FromRawJsonValue<'a> for Vec<T> {
    fn from_raw_json_value(raw: RawJsonValue<'a>) -> Result<Self, JsonParseError> {
        raw.to_array_values()?.map(T::from_raw_json_value).collect()
    }
}

// TODO: Add impl for Cow<'a, str>, String, u8, i8, f32, f64, Option<T>, Vec<T>, [T; N], HashMap etc

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JsonValueIndexEntry {
    pub kind: JsonValueKind,
    pub escaped: bool,
    pub text: Range<usize>,
    pub end_index: usize,
}

#[derive(Debug)]
pub struct RawJson<'a> {
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> RawJson<'a> {
    pub fn parse(text: &'a str) -> Result<Self, JsonParseError> {
        let values = JsonParser::new(text).parse()?;
        Ok(Self { text, values })
    }

    pub fn value(&self) -> RawJsonValue {
        RawJsonValue {
            json: self,
            index: 0,
        }
    }

    pub fn get_value_by_position(&self, position: usize) -> Option<RawJsonValue> {
        let mut value = self.value();
        if !value.entry().text.contains(&position) {
            return None;
        }
        while let Some(child) = Children::new(value).find(|c| c.entry().text.contains(&position)) {
            value = child;
        }
        Some(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawJsonValue<'a> {
    json: &'a RawJson<'a>,
    index: usize,
}

impl<'a> RawJsonValue<'a> {
    pub fn kind(self) -> JsonValueKind {
        self.json.values[self.index].kind
    }

    fn entry(&self) -> &JsonValueIndexEntry {
        &self.json.values[self.index]
    }

    pub fn text(self) -> &'a str {
        let text = &self.json.values[self.index].text;
        &self.json.text[text.start..text.end]
    }

    pub fn position(self) -> usize {
        self.json.values[self.index].text.start
    }

    // TODO: move?
    pub fn to_invalid_value_error<E>(self, error: E) -> JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        JsonParseError::InvalidValue {
            kind: self.kind(),
            position: self.position(),
            error: error.into(),
        }
    }

    pub fn to_unquoted_text(self) -> Cow<'a, str> {
        if self.entry().escaped {
            let mut unescaped = String::with_capacity(self.text().len());
            let mut chars = self.text().chars();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => {
                        let c = chars.next().expect("infallible");
                        match c {
                            '\\' | '/' | '"' | 'n' | 't' | 'r' | 'b' | 'f' => unescaped.push(c),
                            'u' => {
                                let c = std::str::from_utf8(&[
                                    chars.next().expect("infallible") as u8,
                                    chars.next().expect("infallible") as u8,
                                    chars.next().expect("infallible") as u8,
                                    chars.next().expect("infallible") as u8,
                                ])
                                .ok()
                                .and_then(|code| u32::from_str_radix(code, 16).ok())
                                .and_then(char::from_u32)
                                .expect("infallible");
                                unescaped.push(c);
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unescaped.push(c),
                }
            }
            Cow::Owned(unescaped)
        } else {
            Cow::Borrowed(self.text())
        }
    }

    pub fn non_null_then<F, T>(self, f: F) -> Option<T>
    where
        F: FnOnce(Self) -> T,
    {
        (self.kind() != JsonValueKind::Null).then(|| f(self))
    }

    pub fn non_null_then_try<F, T, E>(self, f: F) -> Result<Option<T>, E>
    where
        F: FnOnce(Self) -> Result<T, E>,
    {
        self.non_null_then(f).transpose()
    }

    pub fn try_to<T: FromRawJsonValue<'a>>(self) -> Result<T, JsonParseError> {
        T::from_raw_json_value(self)
    }

    pub fn parse<T>(self) -> Result<T, JsonParseError>
    where
        T: FromStr,
        T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        self.parse_with(|text| text.parse())
    }

    pub fn parse_with<F, T, E>(self, f: F) -> Result<T, JsonParseError>
    where
        F: FnOnce(&str) -> Result<T, E>,
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        f(&self.to_unquoted_text()).map_err(|e| JsonParseError::InvalidValue {
            kind: self.kind(),
            position: self.position(),
            error: e.into(),
        })
    }

    pub fn expect(self, kinds: &'static [JsonValueKind]) -> Result<Self, JsonParseError> {
        if kinds.contains(&self.kind()) {
            Ok(self)
        } else {
            Err(self.to_invalid_value_error(format!(
                "expected {:?}, but found {:?}",
                if kinds.len() == 1 {
                    format!("{:?}", kinds[0])
                } else {
                    format!("one of {:?}", kinds)
                },
                self.kind()
            )))
        }
    }

    pub fn as_bool(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::Bool])
    }

    pub fn as_integer(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::Integer])
    }

    pub fn as_number(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::Integer, JsonValueKind::Float])
    }

    pub fn as_string(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::String])
    }

    pub fn to_array_values(self) -> Result<impl Iterator<Item = RawJsonValue<'a>>, JsonParseError> {
        self.expect(&[JsonValueKind::Array]).map(Children::new)
    }

    pub fn to_fixed_array<const N: usize>(self) -> Result<[RawJsonValue<'a>; N], JsonParseError> {
        let mut values = self.to_array_values()?;
        let mut fixed_array = [self; N];
        for (i, v) in fixed_array.iter_mut().enumerate() {
            *v = values.next().ok_or_else(|| {
                self.to_invalid_value_error(format!(
                    "expected an array with {N} elements, but got only {i} elements"
                ))
            })?;
        }

        let extra = values.count();
        if extra > 0 {
            return Err(self.to_invalid_value_error(format!(
                "expected an array with {N} elements, but got {} elements",
                N + extra
            )));
        }

        Ok(fixed_array)
    }

    pub fn to_object_members(
        self,
    ) -> Result<impl Iterator<Item = (RawJsonValue<'a>, RawJsonValue<'a>)>, JsonParseError> {
        self.expect(&[JsonValueKind::Object])
            .map(JsonKeyValuePairs::new)
    }

    pub fn to_fixed_object<const N: usize, const M: usize>(
        self,
        required_member_names: [&str; N],
        optional_member_names: [&str; M],
    ) -> Result<([RawJsonValue<'a>; N], [Option<RawJsonValue<'a>>; M]), JsonParseError> {
        let mut required = [self; N];
        let mut optional = [None; M];
        for (k, v) in self.to_object_members()? {
            let k = k.to_unquoted_text();
            if let Some(i) = required_member_names.iter().position(|n| k == *n) {
                required[i] = v;
            } else if let Some(i) = optional_member_names.iter().position(|n| k == *n) {
                optional[i] = Some(v);
            }
        }

        if required.iter().any(|v| v.index != self.index) {
            let missings = required_member_names
                .iter()
                .zip(required.iter())
                .filter(|(_, value)| value.index != self.index)
                .map(|(name, _)| name)
                .collect::<Vec<_>>();
            return Err(self
                .to_invalid_value_error(format!("missing required object members: {missings:?}")));
        }

        Ok((required, optional))
    }
}

#[derive(Debug)]
struct Children<'a> {
    value: RawJsonValue<'a>,
    end_index: usize,
}

impl<'a> Children<'a> {
    fn new(mut value: RawJsonValue<'a>) -> Self {
        let end_index = value.entry().end_index;
        value.index += 1;
        Self { value, end_index }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = RawJsonValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value.index == self.end_index {
            return None;
        }
        let value = self.value;
        self.value.index = value.entry().end_index;
        Some(value)
    }
}

#[derive(Debug)]
struct JsonKeyValuePairs<'a> {
    inner: Children<'a>,
}

impl<'a> JsonKeyValuePairs<'a> {
    fn new(object: RawJsonValue<'a>) -> Self {
        Self {
            inner: Children::new(object),
        }
    }
}

impl<'a> Iterator for JsonKeyValuePairs<'a> {
    type Item = (RawJsonValue<'a>, RawJsonValue<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.next()?;
        let value = self.inner.next().expect("infallible");
        Some((key, value))
    }
}
