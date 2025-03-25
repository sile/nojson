use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use crate::Error;

pub trait Json {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JsonValue {
    Null(Null),
    Bool(bool),
    Number(JsonNumber),
    String(JsonString),
    Array(JsonArray),
    Object(JsonObject),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum JsonNumber {
    Integer(i64),
    Float(f64),
}

impl Json for JsonNumber {}

impl Display for JsonNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNumber::Integer(v) => write!(f, "{v}"),
            JsonNumber::Float(v) => write!(f, "{v}"),
        }
    }
}

impl FromStr for JsonNumber {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            s.parse().map(Self::Float).map_err(Error::from)
        } else {
            s.parse().map(Self::Integer).map_err(Error::from)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonString<T = String>(pub T);

impl<T> Json for JsonString<T> where T: AsRef<str> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonArray<T = Vec<JsonValue>>(pub T);

// impl<T, V> Json for JsonArray<T>
// where
//     T: AsRef<[V]>,
//     V: Json,
// {
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonObject<T = BTreeMap<String, JsonValue>>(pub T);
