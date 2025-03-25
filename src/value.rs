use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use crate::Error;

pub trait Json: Display + FromStr {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
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
