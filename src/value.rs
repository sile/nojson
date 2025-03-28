use std::{collections::BTreeMap, fmt::Display, hash::Hash};

use crate::{DisplayJson, Json};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(FiniteF64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl DisplayJson for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(v) => write!(f, "{v}"),
            Value::Integer(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::String(v) => write!(f, "{}", Json(v)),
            Value::Array(v) => write!(f, "{}", Json(v)),
            Value::Object(vs) => {
                write!(f, "{{")?;
                let mut vs = vs.iter();
                if let Some((k, v)) = vs.next() {
                    write!(f, "{k}:{v}")?;
                }
                for (k, v) in vs {
                    write!(f, ",{k}:{v}")?;
                }
                write!(f, "}}")?;
                Ok(())
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplayJson::fmt(self, f)
    }
}

// TODO:  FromStr

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    pub const fn new(v: f64) -> Option<Self> {
        if v.is_finite() {
            Some(Self(if v == -0.0 { 0.0 } else { v }))
        } else {
            None
        }
    }

    pub const fn get(self) -> f64 {
        self.0
    }
}

impl Eq for FiniteF64 {}

impl Ord for FiniteF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Hash for FiniteF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl DisplayJson for FiniteF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for FiniteF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplayJson::fmt(self, f)
    }
}
