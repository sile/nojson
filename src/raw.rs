use std::{borrow::Cow, ops::Range};

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
#[derive(Debug)]
pub struct RawJson<'a> {
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> RawJson<'a> {
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
    pub fn parse(text: &'a str) -> Result<Self, JsonParseError> {
        let values = JsonParser::new(text).parse()?;
        Ok(Self { text, values })
    }

    /// Returns the original JSON text.
    pub fn text(&self) -> &'a str {
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
    pub fn value(&self) -> RawJsonValue {
        RawJsonValue {
            json: self,
            index: 0,
        }
    }

    // TODO: add doc and test
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JsonValueIndexEntry {
    pub kind: JsonValueKind,
    pub escaped: bool,
    pub text: Range<usize>,
    pub end_index: usize,
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

    pub fn json(self) -> &'a RawJson<'a> {
        self.json
    }

    pub fn position(self) -> usize {
        self.json.values[self.index].text.start
    }

    pub fn as_raw_str(self) -> &'a str {
        let text = &self.json.values[self.index].text;
        &self.json.text[text.start..text.end]
    }

    pub fn as_bool_str(self) -> Result<&'a str, JsonParseError> {
        self.expect(&[JsonValueKind::Bool]).map(|v| v.as_raw_str())
    }

    pub fn as_integer_str(self) -> Result<&'a str, JsonParseError> {
        self.expect(&[JsonValueKind::Integer])
            .map(|v| v.as_raw_str())
    }

    pub fn as_float_str(self) -> Result<&'a str, JsonParseError> {
        self.expect(&[JsonValueKind::Float]).map(|v| v.as_raw_str())
    }

    pub fn as_number_str(self) -> Result<&'a str, JsonParseError> {
        self.expect(&[JsonValueKind::Integer, JsonValueKind::Float])
            .map(|v| v.as_raw_str())
    }

    pub fn to_unquoted_string_str(self) -> Result<Cow<'a, str>, JsonParseError> {
        self.expect(&[JsonValueKind::String])
            .map(|v| v.to_unquoted_str())
    }

    fn to_unquoted_str(self) -> Cow<'a, str> {
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

    pub fn expect(self, kinds: &'static [JsonValueKind]) -> Result<Self, JsonParseError> {
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

    pub fn to_array_values(self) -> Result<impl Iterator<Item = RawJsonValue<'a>>, JsonParseError> {
        self.expect(&[JsonValueKind::Array]).map(Children::new)
    }

    pub fn to_fixed_array<const N: usize>(self) -> Result<[RawJsonValue<'a>; N], JsonParseError> {
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
