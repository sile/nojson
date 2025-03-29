use std::{hash::Hash, num::NonZeroUsize, ops::Range, str::FromStr};

// TODO: private
pub const WHITESPACES: [char; 4] = [' ', '\t', '\r', '\n'];

// TODO: Result

#[derive(Debug)]
pub enum Error {}

pub trait FromJsonStr: Sized {
    fn from_json_str(s: &JsonStr) -> Result<Self, Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsonStrKind {
    Null,
    Bool,
    Number { integer: bool },
    String { escaped: bool },
    Array,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JsonValue {
    kind: JsonStrKind,
    text: Range<usize>,
    size: NonZeroUsize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonStr<'t> {
    text: &'t str,
    index: usize,
    values: Vec<JsonValue>,
}

impl<'t> PartialOrd for JsonStr<'t> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'t> Ord for JsonStr<'t> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.text, self.index).cmp(&(other.text, other.index))
    }
}

impl<'t> Hash for JsonStr<'t> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.text, self.index).hash(state);
    }
}

impl<'t> JsonStr<'t> {
    pub fn new(text: &'t str) -> Result<Self, Error> {
        Ok(Self {
            text,
            index: 0,
            values: Vec::new(),
        })
    }

    pub fn text(&self) -> &'t str {
        self.text
    }

    pub fn nullable<F, T>(&self, _f: F) -> Result<Option<T>, Error>
    where
        F: FnOnce() -> Result<T, Error>,
    {
        todo!()
    }

    pub fn parse_integer<T>(&self) -> Result<T, Error>
    where
        T: FromStr,
        Error: From<T::Err>,
    {
        self.parse_integer_with(|s| s.parse())
    }

    pub fn parse_integer_with<F, T, E>(&self, _f: F) -> Result<T, Error>
    where
        F: FnOnce(&str) -> Result<T, E>,
        Error: From<E>,
    {
        todo!()
    }

    pub fn expect_array(&self) -> Result<JsonArrayStr<'_, 't>, Error> {
        todo!()
    }

    // TODO: kind()
    // TODO: parse_null(), parse_bool(), parse_float(), parse_number(), parse_string()
    // TODO: array(), object()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonArrayStr<'a, 't> {
    s: &'a JsonStr<'t>,
}

impl<'a, 't> JsonArrayStr<'a, 't> {
    pub fn get(&self, _index: usize) -> Result<&'a JsonStr<'t>, Error> {
        todo!()
    }
}

impl<'a, 't> Iterator for JsonArrayStr<'a, 't> {
    type Item = &'a JsonStr<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonObjectStr<'a> {
    json: &'a str,
}
