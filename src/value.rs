use std::{collections::BTreeMap, fmt::Display, hash::Hash, str::FromStr};

use crate::Error;

pub trait Json {}

pub trait AsJson {
    type Item: Json;

    fn as_json(&self) -> &Self::Item;
}

pub trait ToJson {
    type Item: Json;

    fn to_json(&self) -> Self::Item;
}

pub trait TryIntoJson {
    type Item: Json;

    fn try_into_json(self) -> Result<Self::Item, Error>;
}

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
// TODO: impl<T> Json for JsonString<T> where T: AsRef<str> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonArray<T = Vec<JsonValue>>(pub T);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonObject<T = BTreeMap<String, JsonValue>>(pub T);
