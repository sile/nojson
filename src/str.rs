use std::{borrow::Cow, hash::Hash, ops::Range};

// TODO: private
pub const WHITESPACES: [char; 4] = [' ', '\t', '\r', '\n'];

// TODO: Result

#[derive(Debug)]
pub struct Error<'text, 'index> {
    pub kind: ErrorKind,
    pub context: JsonStr<'text, 'index>, // TODO: location or else
    pub reason: Option<Box<dyn Send + Sync + std::error::Error>>,
}

impl<'text, 'index> Error<'text, 'index> {
    pub fn new<E>(kind: ErrorKind, context: &JsonStr<'text, 'index>, reason: E) -> Self
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

pub trait FromJsonStr: Sized {
    fn from_json_str<'text, 'index>(
        s: &JsonStr<'text, 'index>,
    ) -> Result<Self, Error<'text, 'index>>;
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

// TODO: rename
#[derive(Debug, Clone, PartialEq, Eq)]
struct JsonValue {
    kind: JsonStrKind,
    text: Range<usize>,
    value: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonStr2<'a> {
    text: &'a str,
    values: Vec<JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValueStr<'a> {
    json: &'a JsonStr2<'a>,
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonStr<'text, 'index> {
    text: Cow<'text, str>,
    index: usize,
    values: Cow<'index, [JsonValue]>,
}

impl<'text, 'index> PartialOrd for JsonStr<'text, 'index> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'text, 'index> Ord for JsonStr<'text, 'index> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.text, self.index).cmp(&(&other.text, other.index))
    }
}

impl<'text, 'index> Hash for JsonStr<'text, 'index> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (&self.text, self.index).hash(state);
    }
}

impl<'text> JsonStr<'text, 'static> {
    pub fn new(text: &'text str) -> Result<Self, Error<'text, 'static>> {
        Ok(Self {
            text: Cow::Borrowed(text),
            index: 0,
            values: Cow::Owned(Vec::new()),
        })
    }
}

impl<'text, 'index> JsonStr<'text, 'index> {
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
