use std::{collections::BTreeMap, fmt::Display, hash::Hash};

use crate::{Json, fmt::DisplayJson, num::FiniteF64};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(FiniteF64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl DisplayJson for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(v) => write!(f, "{v}"),
            JsonValue::Integer(v) => write!(f, "{v}"),
            JsonValue::Float(v) => write!(f, "{v}"),
            JsonValue::String(v) => write!(f, "{}", Json(v)),
            JsonValue::Array(v) => write!(f, "{}", Json(v)),
            JsonValue::Object(v) => write!(f, "{}", Json(v)),
        }
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplayJson::fmt(self, f)
    }
}

// TODO:  FromStr
