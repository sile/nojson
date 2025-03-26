use std::{
    collections::BTreeMap,
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
    str::{FromStr, ParseBoolError},
};

pub mod value;

#[derive(Debug)]
pub enum Error {
    Eos,
    InvalidJsonValue,
    NotNull,
    NotBool,
    NotNumber,
    NotFiniteFloat, // TODO
    NotValidFloat,
}

impl From<ParseBoolError> for Error {
    fn from(_value: ParseBoolError) -> Self {
        Self::NotBool
    }
}

impl From<ParseIntError> for Error {
    fn from(_value: ParseIntError) -> Self {
        Self::NotNumber
    }
}

impl From<ParseFloatError> for Error {
    fn from(_value: ParseFloatError) -> Self {
        Self::NotNumber
    }
}

// TODO: Maybe<T>

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JsonNumber {
    Integer(i64),
    Float(f64),
}

impl FromStr for JsonNumber {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = if s.contains('.') {
            s.parse().map(Self::Integer)?
        } else {
            s.parse().map(Self::Float)?
        };
        Ok(n)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Null;

impl FromStr for Null {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "null" {
            Ok(Self)
        } else {
            Err(Error::NotNull)
        }
    }
}

pub trait ParseOption {
    fn parse_option<T: FromStr>(&self) -> Result<Option<T>, T::Err>;
}

impl ParseOption for str {
    fn parse_option<T: FromStr>(&self) -> Result<Option<T>, T::Err> {
        if self == "null" {
            Ok(None)
        } else {
            self.parse().map(Some)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonString(pub String);

impl FromStr for JsonString {
    type Err = Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(JsonString),
    Array(Vec<JsonValue>),               // TODO: JsonArray
    Object(BTreeMap<String, JsonValue>), // TODO: JsonObject
}

impl FromStr for JsonValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().ok_or(Error::Eos)?;
        let value = match JsonValueKind::from_char(c).ok_or(Error::InvalidJsonValue)? {
            JsonValueKind::Null => s.parse().map(|Null| Self::Null)?,
            JsonValueKind::Bool => s.parse().map(Self::Bool)?,
            JsonValueKind::Number => s.parse().map(Self::Number)?,
            JsonValueKind::String => s.parse().map(Self::String)?,
            JsonValueKind::Array => JsonArrayElements::new(s)
                .map(|s| s.and_then(|s| s.parse()))
                .collect::<Result<_, Error>>()
                .map(Self::Array)?,
            JsonValueKind::Object => JsonObjectMembers::new(s)
                .map(|s| s.and_then(|(k, v)| v.parse().map(|v| (k.to_owned(), v))))
                .collect::<Result<_, Error>>()
                .map(Self::Object)?,
        };
        Ok(value)
    }
}

#[derive(Debug)]
pub struct JsonObjectMemberAccessor<'a> {
    #[expect(dead_code)]
    text: &'a str,
}

impl<'a> JsonObjectMemberAccessor<'a> {
    pub fn get(&self, _name: &str) -> Option<&'a str> {
        todo!()
    }

    pub fn parse<T: FromStr>(&self, _name: &str) -> Result<T, T::Err> {
        todo!()
    }

    pub fn parse_option<T: FromStr>(&self, _name: &str) -> Result<Option<T>, T::Err> {
        todo!()
    }

    pub fn finsh(self) -> &'a str {
        todo!()
    }
}

#[derive(Debug)]
pub struct JsonObjectBuilder {
    text: String,
    first: bool,
}

impl JsonObjectBuilder {
    pub fn new(mut text: String) -> Self {
        text.push('{');
        Self { text, first: true }
    }

    // TODO: -> Result
    // TODO: check when writing value (if valid JSON)
    // TODO: "key".as_json() -> JsonStr<'a>
    pub fn push<K, V>(&mut self, _key: &str, _value: V)
    where
        V: Display,
    {
        use std::fmt::Write;

        if !self.first {
            self.text.push(',');
            self.first = false;
        }

        let _ = write!(&mut self.text, "foo");

        // let mut obj = JsonObjectBuilder::new(String::new());
        // obj.push("foo", 1);
        // obj.push("bar", 2);
        // obj.push_with("baz", |writer| ...);
        // let buf = obj.finish();

        todo!()
    }

    pub fn finish(mut self) -> String {
        // -> RawJsonValue?
        self.text.push('}');
        self.text
    }
}

#[derive(Debug)]
pub struct JsonObjectMembers<'a> {
    #[expect(dead_code)]
    text: &'a str,
}

impl<'a> JsonObjectMembers<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Iterator for JsonObjectMembers<'a> {
    type Item = Result<(&'a str, &'a str), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug)]
pub struct JsonArrayElements<'a> {
    #[expect(dead_code)]
    text: &'a str,
}

impl<'a> JsonArrayElements<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Iterator for JsonArrayElements<'a> {
    type Item = Result<&'a str, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JsonValueKind {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl JsonValueKind {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'n' => Some(Self::Null),
            't' | 'f' => Some(Self::Bool),
            '0'..='9' => Some(Self::Number),
            '"' => Some(Self::String),
            '[' => Some(Self::Array),
            '{' => Some(Self::Object),
            _ => None,
        }
    }
}
