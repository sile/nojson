use std::{borrow::Cow, hash::Hash, num::NonZeroUsize, ops::Range};

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

#[derive(Debug)]
pub struct Error2<'text, 'index> {
    pub kind: ErrorKind,
    pub context: JsonStr3<'text, 'index>, // TODO: location or else
    pub reason: Option<Box<dyn Send + Sync + std::error::Error>>,
}

impl<'text, 'index> Error2<'text, 'index> {
    pub fn new<E>(kind: ErrorKind, context: &JsonStr3<'text, 'index>, reason: E) -> Self
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        Self {
            kind,
            context: context.clone(),
            reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {}

pub trait FromJsonValueStr: Sized {
    fn from_json_value_str(s: &JsonValueStr) -> Result<Self, Error>;
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
struct JsonValueIndexEntry {
    kind: JsonStrKind,
    text: Range<usize>,
    size: NonZeroUsize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonStr<'a> {
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValueStr<'a> {
    json: &'a JsonStr<'a>,
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonStr3<'text, 'index> {
    text: Cow<'text, str>,
    index: usize,
    values: Cow<'index, [JsonValueIndexEntry]>,
}

impl<'text, 'index> PartialOrd for JsonStr3<'text, 'index> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'text, 'index> Ord for JsonStr3<'text, 'index> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.text, self.index).cmp(&(&other.text, other.index))
    }
}

impl<'text, 'index> Hash for JsonStr3<'text, 'index> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (&self.text, self.index).hash(state);
    }
}

impl<'text> JsonStr3<'text, 'static> {
    pub fn new(text: &'text str) -> Result<Self, Error2<'text, 'static>> {
        Ok(Self {
            text: Cow::Borrowed(text),
            index: 0,
            values: Cow::Owned(Vec::new()),
        })
    }
}

impl<'text, 'index> JsonStr3<'text, 'index> {
    pub fn text(&self) -> &str {
        &self.text
    }

    // TODO: remainin_text()

    pub fn kind(&self) -> JsonStrKind {
        self.values[self.index].kind
    }

    // pub fn nullable<F, T>(&self, _f: F) -> Result<Option<T>, Error>
    // where
    //     F: FnOnce() -> Result<T, Error>,
    // {
    //     todo!()
    // }

    // pub fn parse_integer<T>(&self) -> Result<T, Error>
    // where
    //     T: FromStr,
    //     Error: From<T::Err>,
    // {
    //     self.parse_integer_with(|s| s.parse())
    // }

    // pub fn parse_integer_with<F, T, E>(&self, _f: F) -> Result<T, Error>
    // where
    //     F: FnOnce(&str) -> Result<T, E>,
    //     Error: From<E>,
    // {
    //     todo!()
    // }

    // pub fn expect_array(&self) -> Result<JsonArrayStr<'_, 'text>, Error> {
    //     todo!()
    // }

    // TODO: kind()
    // TODO: parse_null(), parse_bool(), parse_float(), parse_number(), parse_string()
    // TODO: array(), object()
}

//#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct JsonArrayStr<'a, 't> {
//     s: &'a JsonStr<'t>,
// }

// impl<'a, 't> JsonArrayStr<'a, 't> {
//     pub fn get(&self, _index: usize) -> Result<&'a JsonStr<'t>, Error> {
//         todo!()
//     }
// }

// impl<'a, 't> Iterator for JsonArrayStr<'a, 't> {
//     type Item = &'a JsonStr<'t>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct JsonObjectStr<'a> {
//     json: &'a str,
// }
