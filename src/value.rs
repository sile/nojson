use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(JsonString),
    Array(JsonArray),
    Object(JsonObject),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JsonNumber {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonString<T = String>(pub T);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonArray<T = Vec<JsonValue>>(pub T);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsonObject<T = BTreeMap<String, JsonValue>>(pub T);
