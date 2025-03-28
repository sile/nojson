use std::str::FromStr;

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
    Integer,
    Float,
    String,
    StringEscaped,
    Array,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonStr<'t> {
    text: &'t str,
}

impl<'t> JsonStr<'t> {
    pub fn new(text: &'t str) -> Result<Self, Error> {
        Ok(Self { text })
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

    pub fn as_array(&self) -> Result<JsonArrayStr<'_, 't>, Error> {
        todo!()
    }

    // pub fn with_array_indices()
    // pub fn with_object_members()

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
