use std::{borrow::Cow, hash::Hash, num::NonZeroUsize, ops::Range, str::FromStr};

// TODO: private
pub const WHITESPACES: [char; 4] = [' ', '\t', '\r', '\n'];

// TODO: Result

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub context: ErrorContext,
    pub reason: Option<Box<dyn Send + Sync + std::error::Error>>,
}

#[derive(Debug)]
pub struct ErrorContext {
    pub line: NonZeroUsize,
    pub column: NonZeroUsize,
    pub path: Vec<()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsonValueStrKind {
    Null,
    Bool,
    Number { integer: bool },
    String { escaped: bool },
    Array,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JsonValueIndexEntry {
    kind: JsonValueStrKind,
    text: Range<usize>,
    size: NonZeroUsize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonStr<'a> {
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonStr<'a> {
    pub fn value(&self) -> JsonValueStr {
        JsonValueStr {
            json: self,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonValueStr<'a> {
    json: &'a JsonStr<'a>,
    index: usize,
}

impl<'a> JsonValueStr<'a> {
    pub fn kind(&self) -> JsonValueStrKind {
        self.json.values[self.index].kind
    }

    pub fn nullable<F, T, E>(&self, f: F) -> Result<Option<T>, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        (self.kind() != JsonValueStrKind::Null).then(f).transpose()
    }

    pub fn parse_integer<T>(&self) -> Result<T, Error>
    where
        T: FromStr,
        T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        self.parse_integer_with(|text| text.parse())
    }

    pub fn parse_integer_with<F, T, E>(&self, _f: F) -> Result<T, Error>
    where
        F: FnOnce(&str) -> Result<T, E>,
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        todo!()
    }

    pub fn array_values(&self) -> Result<JsonArrayValues, Error> {
        todo!()
    }

    pub fn object_members(&self) -> Result<JsonObjectMembers, Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct JsonArrayValues<'a> {
    _value: JsonValueStr<'a>,
}

impl<'a> JsonArrayValues<'a> {
    pub fn get(&self, _index: usize) -> Option<JsonValueStr<'a>> {
        todo!()
    }

    pub fn expect(&self, _index: usize) -> Result<JsonValueStr<'a>, Error> {
        todo!()
    }
}

impl<'a> Iterator for JsonArrayValues<'a> {
    type Item = JsonValueStr<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug)]
pub struct JsonObjectMembers<'a> {
    _value: JsonValueStr<'a>,
}

impl<'a> JsonObjectMembers<'a> {
    pub fn expect(&self, _name: &str) -> Result<JsonValueStr<'a>, Error> {
        todo!()
    }
}

impl<'a> Iterator for JsonObjectMembers<'a> {
    type Item = (Cow<'a, str>, JsonValueStr<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
