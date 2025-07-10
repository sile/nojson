use std::{borrow::Cow, fmt::Display, hash::Hash, ops::Range};

use crate::{parse::JsonParser, DisplayJson, JsonFormatter, JsonValueKind};

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

impl DisplayJson for RawJson<'_> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        DisplayJson::fmt(&self.value(), f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JsonValueIndexEntry {
    pub kind: JsonValueKind,
    pub escaped: bool,
    pub text: Range<usize>,
    pub end_index: usize,
}

/// A JSON value in a [`RawJson`].
///
/// This struct provides the text and structural information (e.g., kind, parent, children) of a JSON value.
/// Interpreting that text is the responsibility of the user.
///
/// To convert this JSON value to a Rust type, you can use the standard [`TryFrom`] and [`TryInto`] traits.
/// For other parsing approaches, you can use the [`FromStr`](std::str::FromStr) trait or other parsing methods
/// to parse the underlying JSON text of this value as shown below:
///
/// ```
/// # use nojson::{RawJson, RawJsonValue, JsonParseError};
/// # fn main() -> Result<(), JsonParseError> {
/// let text = "1.23";
/// let json = RawJson::parse(text)?;
/// let raw: RawJsonValue = json.value();
/// let parsed: f32 =
///     raw.as_number_str()?.parse().map_err(|e| raw.invalid(e))?;
/// assert_eq!(parsed, 1.23);
/// # Ok(())
/// # }
/// ```
///
/// For types that implement `TryFrom<RawJsonValue<'_, '_>>`, you can use the [`TryInto`] trait:
///
/// ```
/// # use nojson::{RawJson, JsonParseError};
/// # fn main() -> Result<(), JsonParseError> {
/// let json = RawJson::parse("[1, 2, 3]")?;
/// let numbers: [u32; 3] = json.value().try_into()?;
/// assert_eq!(numbers, [1, 2, 3]);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawJsonValue<'text, 'raw> {
    index: usize,
    json: &'raw RawJson<'text>,
}

impl<'text, 'raw> RawJsonValue<'text, 'raw> {
    /// Returns the kind of this JSON value.
    pub fn kind(self) -> JsonValueKind {
        self.json.values[self.index].kind
    }

    /// Returns the byte position where this value begins in the JSON text (`self.json().text()`).
    pub fn position(self) -> usize {
        self.json.values[self.index].text.start
    }

    /// Returns a reference to the [`RawJson`] instance that contains this value.
    pub fn json(self) -> &'raw RawJson<'text> {
        self.json
    }

    /// Returns the parent value (array or object) that contains this value.
    pub fn parent(self) -> Option<Self> {
        if self.index == 0 {
            return None;
        }
        self.json.get_value_by_position(self.position() - 1)
    }

    /// Returns the raw JSON text of this value as-is.
    pub fn as_raw_str(self) -> &'text str {
        let text = &self.json.values[self.index].text;
        &self.json.text[text.start..text.end]
    }

    /// Similar to [`RawJsonValue::as_raw_str()`],
    /// but this method verifies whether the value is a JSON boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("false")?;
    /// assert_eq!(json.value().as_boolean_str()?.parse(), Ok(false));
    ///
    /// let json = RawJson::parse("10")?;
    /// assert!(json.value().as_boolean_str().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_boolean_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Boolean])
            .map(|v| v.as_raw_str())
    }

    /// Similar to [`RawJsonValue::as_raw_str()`],
    /// but this method verifies whether the value is a JSON integer number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("123")?;
    /// assert_eq!(json.value().as_integer_str()?.parse(), Ok(123));
    ///
    /// let json = RawJson::parse("12.3")?;
    /// assert!(json.value().as_integer_str().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_integer_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Integer])
            .map(|v| v.as_raw_str())
    }

    /// Similar to [`RawJsonValue::as_raw_str()`],
    /// but this method verifies whether the value is a JSON floating-point number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("12.3")?;
    /// assert_eq!(json.value().as_float_str()?.parse(), Ok(12.3));
    ///
    /// let json = RawJson::parse("123")?;
    /// assert!(json.value().as_float_str().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_float_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Float]).map(|v| v.as_raw_str())
    }

    /// Similar to [`RawJsonValue::as_raw_str()`],
    /// but this method verifies whether the value is a JSON number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("123")?;
    /// assert_eq!(json.value().as_number_str()?.parse(), Ok(123));
    ///
    /// let json = RawJson::parse("12.3")?;
    /// assert_eq!(json.value().as_number_str()?.parse(), Ok(12.3));
    ///
    /// let json = RawJson::parse("null")?;
    /// assert!(json.value().as_number_str().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_number_str(self) -> Result<&'text str, JsonParseError> {
        self.expect([JsonValueKind::Integer, JsonValueKind::Float])
            .map(|v| v.as_raw_str())
    }

    /// Similar to [`RawJsonValue::as_raw_str()`],
    /// but this method verifies whether the value is a JSON string and returns the unquoted content of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("\"123\"")?;
    /// assert_eq!(json.value().to_unquoted_string_str()?, "123");
    /// assert_eq!(json.value().to_unquoted_string_str()?.parse(), Ok(123));
    ///
    /// let json = RawJson::parse("123")?;
    /// assert!(json.value().to_unquoted_string_str().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_unquoted_string_str(self) -> Result<Cow<'text, str>, JsonParseError> {
        self.expect([JsonValueKind::String]).map(|v| v.unquote())
    }

    /// If the value is a JSON array,
    /// this method returns an iterator that iterates over the array's elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("[0, 1, 2]")?;
    /// for (i, v) in json.value().to_array()?.enumerate() {
    ///     assert_eq!(v.as_integer_str()?.parse(), Ok(i));
    /// }
    ///
    /// let json = RawJson::parse("null")?;
    /// assert!(json.value().to_array().is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// For converting to a fixed-size array, you can use the `TryInto` trait instead:
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("[0, 1, 2]")?;
    /// let fixed_array: [usize; 3] = json.value().try_into()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_array(self) -> Result<impl Iterator<Item = Self>, JsonParseError> {
        self.expect([JsonValueKind::Array]).map(Children::new)
    }

    /// If the value is a JSON object,
    /// this method returns an iterator that iterates over
    /// the name and value pairs of the object's members.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"a": 1, "b": 2, "c": 3}"#)?;
    /// let mut members = json.value().to_object()?;
    /// let (k, v) = members.next().expect("some");
    /// assert_eq!(k.to_unquoted_string_str()?, "a");
    /// assert_eq!(v.as_integer_str()?.parse(), Ok(1));
    ///
    /// let json = RawJson::parse("null")?;
    /// assert!(json.value().to_object().is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_object(self) -> Result<impl Iterator<Item = (Self, Self)>, JsonParseError> {
        self.expect([JsonValueKind::Object])
            .map(JsonKeyValuePairs::new)
    }

    /// Attempts to access a member of a JSON object by name.
    ///
    /// This method returns a [`RawJsonMember`] that represents the result of
    /// looking up the specified member name. The member may or may not exist,
    /// and you can use methods like [`RawJsonMember::required()`] or convert
    /// it to an `Option<T>` to handle both cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"name": "Alice", "age": 30}"#)?;
    /// let obj = json.value();
    ///
    /// // Access existing member
    /// let name_value: String = obj.to_member("name")?.required()?.try_into()?;
    /// assert_eq!(name_value, "Alice");
    ///
    /// // Handle optional member
    /// let city_member = obj.to_member("city")?;
    /// let city: Option<String> = city_member.try_into()?;
    /// assert_eq!(city, None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance
    ///
    /// This method has O(n) complexity where n is the number of members in the object,
    /// as it performs a linear search through all object members to find the requested name.
    /// If you need to access multiple members from the same object, consider using
    /// [`RawJsonValue::to_object()`] instead, which allows you to iterate through
    /// all members once and extract the values you need more efficiently.
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"name": "Alice", "age": 30, "city": "New York"}"#)?;
    /// let obj = json.value();
    ///
    /// // Efficient: single iteration for multiple members
    /// let mut name = None;
    /// let mut age = None;
    /// let mut city = None;
    /// for (key, value) in obj.to_object()? {
    ///     match key.to_unquoted_string_str()?.as_ref() {
    ///         "name" => name = Some(value),
    ///         "age" => age = Some(value),
    ///         "city" => city = Some(value),
    ///         _ => {}
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_member<'a>(
        self,
        name: &'a str,
    ) -> Result<RawJsonMember<'text, 'raw, 'a>, JsonParseError> {
        let member = self
            .to_object()?
            .find(|(key, _)| key.unquote() == name)
            .map(|(_, value)| value);

        Ok(RawJsonMember {
            object: self,
            name,
            member,
        })
    }

    /// Creates a [`JsonParseError::InvalidValue`] error for this value.
    ///
    /// This is a convenience method that's equivalent to calling
    /// [`JsonParseError::invalid_value()`] with this value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse("\"not_a_number\"")?;
    /// let value = json.value();
    ///
    /// // These are equivalent:
    /// let error1 = value.invalid("expected a number");
    /// let error2 = nojson::JsonParseError::invalid_value(value, "expected a number");
    /// # Ok(())
    /// # }
    /// ```
    pub fn invalid<E>(self, error: E) -> JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        JsonParseError::invalid_value(self, error)
    }

    fn unquote(self) -> Cow<'text, str> {
        debug_assert!(self.kind().is_string());

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
                        '\\' | '/' | '"' => unescaped.push(c),
                        'n' => unescaped.push('\n'),
                        't' => unescaped.push('\t'),
                        'r' => unescaped.push('\r'),
                        'b' => unescaped.push('\u{8}'),
                        'f' => unescaped.push('\u{c}'),
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
            Err(self.invalid(format!(
                "expected {}, but found {:?}",
                if kinds.len() == 1 {
                    format!("{:?}", kinds[0])
                } else {
                    format!("one of {kinds:?}")
                },
                self.kind()
            )))
        }
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

impl DisplayJson for RawJsonValue<'_, '_> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        match self.kind() {
            JsonValueKind::Null
            | JsonValueKind::Boolean
            | JsonValueKind::Integer
            | JsonValueKind::Float => write!(f.inner_mut(), "{}", self.as_raw_str()),
            JsonValueKind::String => f.string(self.unquote()),
            JsonValueKind::Array => f.array(|f| f.elements(self.to_array().expect("infallible"))),
            JsonValueKind::Object => f.object(|f| f.members(self.to_object().expect("infallible"))),
        }
    }
}

#[derive(Debug)]
struct Children<'text, 'raw> {
    value: RawJsonValue<'text, 'raw>,
    end_index: usize,
}

impl<'text, 'raw> Children<'text, 'raw> {
    fn new(mut value: RawJsonValue<'text, 'raw>) -> Self {
        let end_index = value.entry().end_index;
        value.index += 1;
        Self { value, end_index }
    }
}

impl<'text, 'raw> Iterator for Children<'text, 'raw> {
    type Item = RawJsonValue<'text, 'raw>;

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
struct JsonKeyValuePairs<'text, 'raw> {
    inner: Children<'text, 'raw>,
}

impl<'text, 'raw> JsonKeyValuePairs<'text, 'raw> {
    fn new(object: RawJsonValue<'text, 'raw>) -> Self {
        Self {
            inner: Children::new(object),
        }
    }
}

impl<'text, 'raw> Iterator for JsonKeyValuePairs<'text, 'raw> {
    type Item = (RawJsonValue<'text, 'raw>, RawJsonValue<'text, 'raw>);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.next()?;
        let value = self.inner.next().expect("infallible");
        Some((key, value))
    }
}

/// Represents a member access result for a JSON object.
///
/// This struct is returned by [`RawJsonValue::to_member()`] and allows you to handle
/// both present and missing object members. It wraps an optional value that is
/// `Some` if the member exists and `None` if it doesn't.
///
/// # Examples
///
/// ```
/// # use nojson::RawJson;
/// # fn main() -> Result<(), nojson::JsonParseError> {
/// let json = RawJson::parse(r#"{"name": "Alice", "age": 30}"#)?;
/// let obj = json.value();
///
/// // Access an existing member
/// let name_member = obj.to_member("name")?;
/// let name: String = name_member.required()?.try_into()?;
/// assert_eq!(name, "Alice");
///
/// // Access a missing member
/// let city_member = obj.to_member("city")?;
/// let city: Option<String> = city_member.try_into()?;
/// assert_eq!(city, None);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawJsonMember<'text, 'raw, 'a> {
    object: RawJsonValue<'text, 'raw>,
    name: &'a str,
    member: Option<RawJsonValue<'text, 'raw>>,
}

impl<'text, 'raw, 'a> RawJsonMember<'text, 'raw, 'a> {
    /// Returns the member value if it exists, or an error if it's missing.
    ///
    /// This method is useful when you need to ensure that a required member
    /// is present in the JSON object.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"name": "Alice"}"#)?;
    /// let obj = json.value();
    ///
    /// // Required member exists
    /// let name = obj.to_member("name")?.required()?;
    /// assert_eq!(name.to_unquoted_string_str()?, "Alice");
    ///
    /// // Required member missing - returns error
    /// let age_result = obj.to_member("age")?.required();
    /// assert!(age_result.is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn required(self) -> Result<RawJsonValue<'text, 'raw>, JsonParseError> {
        self.member.ok_or_else(|| {
            self.object
                .invalid(format!("required member '{}' is missing", self.name))
        })
    }

    /// Returns the inner raw JSON value as an `Option`.
    ///
    /// This method provides direct access to the underlying `Option<RawJsonValue>`,
    /// allowing you to handle the presence or absence of the member yourself.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"name": "Alice", "age": 30}"#)?;
    /// let obj = json.value();
    ///
    /// // Existing member
    /// let name_member = obj.to_member("name")?;
    /// if let Some(name_value) = name_member.get() {
    ///     assert_eq!(name_value.to_unquoted_string_str()?, "Alice");
    /// }
    ///
    /// // Missing member
    /// let city_member = obj.to_member("city")?;
    /// assert!(city_member.get().is_none());
    ///
    /// // Using with pattern matching
    /// match obj.to_member("age")?.get() {
    ///     Some(age_value) => {
    ///         let age: i32 = age_value.as_integer_str()?.parse()
    ///             .map_err(|e| age_value.invalid(e))?;
    ///         assert_eq!(age, 30);
    ///     }
    ///     None => println!("Age not provided"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(self) -> Option<RawJsonValue<'text, 'raw>> {
        self.member
    }

    /// Applies a transformation function to the member value if it exists.
    ///
    /// This method is similar to [`Option::map`], but designed for transformations
    /// that can fail with a [`JsonParseError`]. If the member exists, the function
    /// is applied to its value. If the member doesn't exist, `Ok(None)` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"name": "Alice", "age": "30"}"#)?;
    /// let obj = json.value();
    ///
    /// // Transform existing member
    /// let age_member = obj.to_member("age")?;
    /// let age: Option<i32> = age_member.map(|v| {
    ///     v.to_unquoted_string_str()?.parse().map_err(|e| v.invalid(e))
    /// })?;
    /// assert_eq!(age, Some(30));
    ///
    /// // Transform missing member
    /// let city_member = obj.to_member("city")?;
    /// let city: Option<String> = city_member.map(|v| v.try_into())?;
    /// assert_eq!(city, None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// This is particularly useful when you need to perform parsing or validation
    /// on optional members without having to handle the `Option` separately:
    ///
    /// ```
    /// # use nojson::RawJson;
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let json = RawJson::parse(r#"{"score": "95.5"}"#)?;
    /// let obj = json.value();
    ///
    /// // Parse optional numeric string
    /// let score: Option<f64> = obj.to_member("score")?.map(|v| {
    ///     v.to_unquoted_string_str()?.parse().map_err(|e| v.invalid(e))
    /// })?;
    /// assert_eq!(score, Some(95.5));
    /// # Ok(())
    /// # }
    /// ```
    pub fn map<F, T>(self, f: F) -> Result<Option<T>, JsonParseError>
    where
        F: FnOnce(RawJsonValue<'text, 'raw>) -> Result<T, JsonParseError>,
    {
        self.member.map(f).transpose()
    }
}

impl<'text, 'raw, 'a, T> TryFrom<RawJsonMember<'text, 'raw, 'a>> for Option<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonMember<'text, 'raw, 'a>) -> Result<Self, Self::Error> {
        value
            .member
            .map(T::try_from)
            .transpose()
            .map_err(JsonParseError::from)
    }
}
