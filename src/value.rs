use std::{
    collections::BTreeMap,
    fmt::Display,
    hash::Hash,
    str::{Chars, FromStr},
};

use crate::Error;

pub trait Json {}

impl<T: Json> Json for &T {}

impl<T: Json> Json for Box<T> {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(JsonF64),
    String(JsonString),
    Array(JsonArray),
    Object(JsonObject),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Json for Null {}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Maybe<T>(Option<T>);

impl<T: Json + Display> Display for Maybe<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.0 {
            v.fmt(f)
        } else {
            Null.fmt(f)
        }
    }
}

impl<T: Json + FromStr> FromStr for Maybe<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "null" {
            Ok(Self(None))
        } else {
            s.parse().map(Some).map(Self)
        }
    }
}

impl<T: Json> Json for Maybe<T> {}

// TODO: ParseOption

impl Json for bool {}

impl Json for isize {}

impl Json for u8 {}

impl Json for u16 {}

impl Json for u32 {}

impl Json for u64 {}

impl Json for u128 {}

impl Json for usize {}

impl Json for i8 {}

impl Json for i16 {}

impl Json for i32 {}

impl Json for i64 {}

impl Json for i128 {}

// TODO: impl Eq, Hash, Ord
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct JsonF32(f32);

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct JsonF64(f64);

impl JsonF64 {
    pub fn new(n: f64) -> Option<Self> {
        n.is_finite()
            .then(|| if n == -0.0 { Self(0.0) } else { Self(n) })
    }

    pub const fn get(self) -> f64 {
        self.0
    }
}

impl Eq for JsonF64 {}

impl Ord for JsonF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Hash for JsonF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl AsRef<f64> for JsonF64 {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl From<JsonF64> for f64 {
    fn from(value: JsonF64) -> Self {
        value.0
    }
}

impl TryFrom<f64> for JsonF64 {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(Error::NotFiniteFloat)
    }
}

impl Display for JsonF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for JsonF64 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !is_json_float_chars(s) {
            return Err(Error::NotValidFloat);
        }

        // TODO: Don't ignore the cause
        s.parse().map(JsonF64).map_err(|_| Error::NotValidFloat)
    }
}

impl Json for JsonF64 {}

fn is_json_float_chars(s: &str) -> bool {
    let s = s.strip_prefix('-').unwrap_or(s);
    let digit = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    s.starts_with(digit)
        && s.ends_with(digit)
        && s.chars().all(|c| matches!(c, '-' | '.' | '0'..='9'))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonString<T = String>(T);

impl<T> JsonString<T> {
    pub const fn new(s: T) -> Self {
        Self(s)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Display> Display for JsonString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        write!(f, "\"")?;
        write!(JsonStringWriter(f), "{}", self.0)?;
        write!(f, "\"")?;

        Ok(())
    }
}

impl<T: FromStr> FromStr for JsonString<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('"') else {
            return Err(Error::NotString);
        };
        let Some(s) = s.strip_suffix('"') else {
            return Err(Error::NotValidString);
        };

        if s.chars().any(|c| c.is_control() || matches!(c, '"' | '\\')) {
            let mut unescaped = String::with_capacity(s.len());
            let mut chars = s.chars();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => unescaped.push(unescape_char(&mut chars)?),
                    '"' => return Err(Error::NotValidString),
                    _ if c.is_control() => return Err(Error::NotValidString),
                    _ => unescaped.push(c),
                }
            }
            unescaped
                .parse()
                .map(Self)
                .map_err(|_| Error::NotValidString)
        } else {
            // TODO: Don't ignore the cause
            s.parse().map(Self).map_err(|_| Error::NotValidString)
        }
    }
}

impl<T> Json for JsonString<T> {}

fn unescape_char(chars: &mut Chars) -> Result<char, Error> {
    let c = chars.next().ok_or(Error::NotValidString)?;
    match c {
        'n' => Ok('\n'),
        'r' => Ok('\r'),
        't' => Ok('\t'),
        '\\' => Ok('\\'),
        '"' => Ok('"'),
        'u' => {
            let mut code_point = 0;
            for _ in 0..4 {
                let hex_char = chars.next().ok_or(Error::NotValidString)?;
                let digit = hex_char.to_digit(16).ok_or(Error::NotValidString)?;
                code_point = (code_point << 4) | digit;
            }
            char::from_u32(code_point).ok_or(Error::NotValidString)
        }
        _ => Err(Error::NotValidString),
    }
}

struct JsonStringWriter<'a, 'b>(&'a mut std::fmt::Formatter<'b>);

impl<'a, 'b> std::fmt::Write for JsonStringWriter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            match c {
                '\n' => write!(self.0, r#"\n"#)?,
                '\r' => write!(self.0, r#"\r"#)?,
                '\t' => write!(self.0, r#"\t"#)?,
                '\\' => write!(self.0, r#"\\"#)?,
                '\"' => write!(self.0, r#"\""#)?,
                '\u{0008}' => write!(self.0, r#"\b"#)?,
                '\u{000C}' => write!(self.0, r#"\f"#)?,
                c if c.is_control() => write!(self.0, r#"\u{:04x}"#, c as u32)?,
                _ => write!(self.0, "{c}")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonArray<T = JsonValue>(Vec<T>);

impl<T: Json + Display> Display for JsonArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", JsonIter(self.0.iter()))
    }
}

impl<T: Json + FromStr> FromStr for JsonArray<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(WHITESPACES); // TODO
        let s = s.strip_prefix('[').ok_or(Error::NotValidArray)?;
        let s = s.strip_prefix(']').ok_or(Error::NotValidArray)?;
        let s = s.trim_matches(WHITESPACES);
        if s.is_empty() {
            return Ok(Self(Vec::new()));
        }

        let mut array = Vec::new();

        // TODO: string handling
        for value in s.split(',') {
            let value = value.trim_matches(WHITESPACES);
            let value = value.parse().map_err(|_| Error::NotValidArray)?; // TODO
            array.push(value);
        }

        Ok(Self(array))
    }
}

impl<T> Json for JsonArray<T> {}

const WHITESPACES: [char; 4] = [' ', '\t', '\r', '\n'];

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonIter<T>(T);

impl<T> Display for JsonIter<T>
where
    T: Iterator + Clone,
    T::Item: Json + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.0.clone() {
            if first {
                write!(f, "{item}")?;
                first = false;
            } else {
                write!(f, ",{item}")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T> Json for JsonIter<T> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonObject<T = BTreeMap<String, JsonValue>>(pub T);
