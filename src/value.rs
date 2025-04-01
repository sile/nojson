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

impl JsonValue {
    pub fn kind(&self) -> JsonValueKind {
        match self {
            JsonValue::Null => JsonValueKind::Null,
            JsonValue::Bool(_) => JsonValueKind::Bool,
            JsonValue::Integer(_) => JsonValueKind::Integer,
            JsonValue::Float(_) => JsonValueKind::Float,
            JsonValue::String(_) => JsonValueKind::String,
            JsonValue::Array(_) => JsonValueKind::Array,
            JsonValue::Object(_) => JsonValueKind::Object,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsonValueKind {
    Null,
    Bool,
    Integer,
    Float,
    String,
    Array,
    Object,
}

impl JsonValueKind {
    pub const fn is_null(self) -> bool {
        matches!(self, Self::Null)
    }

    pub const fn is_bool(self) -> bool {
        matches!(self, Self::Bool)
    }

    pub const fn is_integer(self) -> bool {
        matches!(self, Self::Integer)
    }

    pub const fn is_float(self) -> bool {
        matches!(self, Self::Float)
    }

    pub const fn is_number(self) -> bool {
        matches!(self, Self::Integer | Self::Float)
    }

    pub const fn is_string(self) -> bool {
        matches!(self, Self::String)
    }

    pub const fn is_array(self) -> bool {
        matches!(self, Self::Array)
    }

    pub const fn is_object(self) -> bool {
        matches!(self, Self::Object)
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            JsonValueKind::Null => "null",
            JsonValueKind::Bool => "boolean",
            JsonValueKind::Integer => "number",
            JsonValueKind::Float => "float",
            JsonValueKind::String => "string",
            JsonValueKind::Array => "array",
            JsonValueKind::Object => "object",
        }
    }
}
