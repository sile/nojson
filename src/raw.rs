use std::{borrow::Cow, fmt::Display, hash::Hash, ops::Range};

use crate::{FromRawJsonValue, JsonValueKind, parse::JsonParser};

pub use crate::parse_error::JsonParseError;

/// Parsed JSON text (syntactically correct, but not yet converted to Rust types).
///
/// This struct holds a JSON text in its original form
/// (i.e., JSON integers are not converted to Rust's integers),
/// while ensuring the text is valid JSON syntax.
///
/// [`RawJson`] maintains index information about each JSON value in the text,
/// including its type ([`JsonValueKind`]) and the start and end byte positions.
/// You can traverse the JSON structure by accessing the top-level value
/// via [`RawJson::value()`], which returns a [`RawJsonValue`]
/// that provides methods to explore nested elements and convert them into Rust types.
///
/// Note that, for simple use cases,
/// using [`Json`](crate::Json), which internally uses [`RawJson`], is a more convenient way to parse JSON text into Rust types.
#[derive(Debug, Clone)]
pub struct RawJson<'text> {
    text: &'text str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'text> RawJson<'text> {
    /// Parses a JSON string into a [`RawJson`] instance.
    ///
    /// This validates the JSON syntax without converting values to Rust types.
    ///
    /// # Example
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let text = r#"{"name": "John", "age": 30}"#;
    /// let json = RawJson::parse(text)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse(text: &'text str) -> Result<Self, JsonParseError> {
        let values = JsonParser::new(text).parse()?;
        Ok(Self { text, values })
    }

    /// Returns the original JSON text.
    pub fn text(&self) -> &'text str {
        self.text
    }

    /// Returns the top-level value of the JSON.
    ///
    /// This value can be used as an entry point to traverse the entire JSON structure
    /// and convert it to Rust types.
    ///
    /// # Example
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let text = r#"{"name": "John", "age": 30}"#;
    /// let json = RawJson::parse(text).unwrap();
    /// let value = json.value();
    /// # Ok(())
    /// # }
    /// ```
    pub fn value(&self) -> RawJsonValue<'text, '_> {
        RawJsonValue {
            json: self,
            index: 0,
        }
    }

    /// Finds the JSON value at the specified byte position in the original text.
    ///
    /// This method traverses the JSON structure to find the most specific value
    /// that contains the given position.
    /// It returns `None` if the position is outside the bounds of the JSON text.
    ///
    /// This method is useful for retrieving the context
    /// where a [`JsonParseError::InvalidValue`] error occurred.
    ///
    /// # Example
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"name": "John", "age": 30}"#)?;
    ///
    /// // Position at "name" key
    /// let name_value = json.get_value_by_position(2).expect("infallible");
    /// assert_eq!(name_value.as_raw_str(), r#""name""#);
    ///
    /// // Position at number value
    /// let age_value = json.get_value_by_position(25).expect("infallible");
    /// assert_eq!(age_value.as_raw_str(), "30");
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_value_by_position(&self, position: usize) -> Option<RawJsonValue<'text, '_>> {
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

impl PartialEq for RawJson<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for RawJson<'_> {}

impl PartialOrd for RawJson<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RawJson<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.text.cmp(other.text)
    }
}

impl Hash for RawJson<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text.hash(state);
    }
}

impl Display for RawJson<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

// TODO: impl DisplayJson

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JsonValueIndexEntry {
    pub kind: JsonValueKind,
    pub escaped: bool,
    pub text: Range<usize>,
    pub end_index: usize,
}

/// A JSON value in a [`RasJson`].
///
/// This struct only provides that the text and kind of this JSON value.
/// How to interpret that text is responsibility of the user.
///
/// To convert this JSON value to a Rust type that implements the [`FromRawJsonValue`],
/// [`RawJsonValue::try_to()`] is convinient.
/// For other Rust types, you can use the standard [`FromStr`] trait or other parsing methods to parse the underlaying JSON text of this value as follows:
///
/// ```
/// # use nojson::{RawJson, RawJsonValue, JsonParseError};
/// # fn main() -> Result<(), JsonParseError> {
/// let text = "1.23";
/// let json = RawJson::parse(text)?;
/// let raw: RawJsonValue = json.value();
/// let parsed: f32 =
///     raw.as_number_str()?.parse().map_err(|e| JsonParseError::invalid_value(raw, e))?;
/// assert_eq!(parsed, 1.23);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawJsonValue<'text, 'a> {
    json: &'a RawJson<'text>,
    index: usize,
}

impl<'text, 'a> RawJsonValue<'text, 'a> {
    /// Returns the kind of this JSON value.
    pub fn kind(self) -> JsonValueKind {
        self.json.values[self.index].kind
    }

    pub fn try_to<T: FromRawJsonValue<'text>>(self) -> Result<T, JsonParseError> {
        T::from_raw_json_value(self)
    }

    pub fn parent(self) -> Option<Self> {
        if self.index == 0 {
            return None;
        }
        self.json.get_value_by_position(self.position() - 1)
    }

    pub fn json(self) -> &'a RawJson<'text> {
        self.json
    }

    pub fn position(self) -> usize {
        self.json.values[self.index].text.start
    }

    pub fn as_raw_str(self) -> &'text str {
        let text = &self.json.values[self.index].text;
        &self.json.text[text.start..text.end]
    }

    pub fn as_bool_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Bool]).map(|v| v.as_raw_str())
    }

    pub fn as_integer_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Integer])
            .map(|v| v.as_raw_str())
    }

    pub fn as_float_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Float]).map(|v| v.as_raw_str())
    }

    pub fn as_number_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Integer, JsonValueKind::Float])
            .map(|v| v.as_raw_str())
    }

    pub fn to_unquoted_string_str(self) -> Result<Cow<'text, str>, JsonParseError> {
        self.expect([JsonValueKind::String])
            .map(|v| v.to_unquoted_str())
    }

    fn to_unquoted_str(self) -> Cow<'text, str> {
        if !self.kind().is_string() {
            return Cow::Borrowed(self.as_raw_str());
        }

        let content = &self.as_raw_str()[1..self.as_raw_str().len() - 1];
        if !self.entry().escaped {
            return Cow::Borrowed(content);
        }

        let mut unescaped = String::with_capacity(content.len());
        let mut chars = content.chars();
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
    }

    fn expect<const N: usize>(self, kinds: [JsonValueKind; N]) -> Result<Self, JsonParseError> {
        if kinds.contains(&self.kind()) {
            Ok(self)
        } else {
            Err(JsonParseError::invalid_value(
                self,
                format!(
                    "expected {}, but found {:?}",
                    if kinds.len() == 1 {
                        format!("{:?}", kinds[0])
                    } else {
                        format!("one of {:?}", kinds)
                    },
                    self.kind()
                ),
            ))
        }
    }

    pub fn to_array_values(
        self,
    ) -> Result<impl Iterator<Item = RawJsonValue<'text, 'a>>, JsonParseError> {
        self.expect([JsonValueKind::Array]).map(Children::new)
    }

    pub fn to_fixed_array<const N: usize>(
        self,
    ) -> Result<[RawJsonValue<'text, 'a>; N], JsonParseError> {
        let mut values = self.to_array_values()?;
        let mut fixed_array = [self; N];
        for (i, v) in fixed_array.iter_mut().enumerate() {
            *v = values.next().ok_or_else(|| {
                JsonParseError::invalid_value(
                    self,
                    format!("expected an array with {N} elements, but got only {i} elements"),
                )
            })?;
        }

        let extra = values.count();
        if extra > 0 {
            return Err(JsonParseError::invalid_value(
                self,
                format!(
                    "expected an array with {N} elements, but got {} elements",
                    N + extra
                ),
            ));
        }

        Ok(fixed_array)
    }

    pub fn to_object_members(
        self,
    ) -> Result<
        impl Iterator<Item = (RawJsonValue<'text, 'a>, RawJsonValue<'text, 'a>)>,
        JsonParseError,
    > {
        self.expect([JsonValueKind::Object])
            .map(JsonKeyValuePairs::new)
    }

    pub fn to_fixed_object<const N: usize, const M: usize>(
        self,
        required_member_names: [&str; N],
        optional_member_names: [&str; M],
    ) -> Result<
        (
            [RawJsonValue<'text, 'a>; N],
            [Option<RawJsonValue<'text, 'a>>; M],
        ),
        JsonParseError,
    > {
        let mut required = [self; N];
        let mut optional = [None; M];
        for (k, v) in self.to_object_members()? {
            let k = k.to_unquoted_str();
            dbg!(&k);
            if let Some(i) = required_member_names.iter().position(|n| k == *n) {
                dbg!(v);
                required[i] = v;
            } else if let Some(i) = optional_member_names.iter().position(|n| k == *n) {
                dbg!(v);
                optional[i] = Some(v);
            }
        }

        if required.iter().any(|v| v.index == self.index) {
            let missings = required_member_names
                .iter()
                .zip(required.iter())
                .filter(|(_, value)| value.index == self.index)
                .map(|(name, _)| name)
                .collect::<Vec<_>>();
            return Err(JsonParseError::invalid_value(
                self,
                format!("missing required object members: {missings:?}"),
            ));
        }

        Ok((required, optional))
    }

    fn entry(&self) -> &JsonValueIndexEntry {
        &self.json.values[self.index]
    }
}

impl Display for RawJsonValue<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_raw_str())
    }
}

// TODO: impl DisplayJson

#[derive(Debug)]
struct Children<'text, 'a> {
    value: RawJsonValue<'text, 'a>,
    end_index: usize,
}

impl<'text, 'a> Children<'text, 'a> {
    fn new(mut value: RawJsonValue<'text, 'a>) -> Self {
        let end_index = value.entry().end_index;
        value.index += 1;
        Self { value, end_index }
    }
}

impl<'text, 'a> Iterator for Children<'text, 'a> {
    type Item = RawJsonValue<'text, 'a>;

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
struct JsonKeyValuePairs<'text, 'a> {
    inner: Children<'text, 'a>,
}

impl<'text, 'a> JsonKeyValuePairs<'text, 'a> {
    fn new(object: RawJsonValue<'text, 'a>) -> Self {
        Self {
            inner: Children::new(object),
        }
    }
}

impl<'text, 'a> Iterator for JsonKeyValuePairs<'text, 'a> {
    type Item = (RawJsonValue<'text, 'a>, RawJsonValue<'text, 'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.next()?;
        let value = self.inner.next().expect("infallible");
        Some((key, value))
    }
}
