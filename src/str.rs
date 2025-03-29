use std::{borrow::Cow, hash::Hash, num::NonZeroUsize, ops::Range, str::FromStr};

const WHITESPACE_PATTERN: [char; 4] = [' ', '\t', '\r', '\n'];

#[derive(Debug)]
pub enum JsonErrorContext<'a> {
    Parser(&'a JsonParse<'a>),
    Value(JsonValueStr<'a>),
}

impl<'a> JsonErrorContext<'a> {
    pub fn line_and_column(&self) -> (NonZeroUsize, NonZeroUsize) {
        todo!()
    }

    pub fn path(&self) -> Vec<()> {
        todo!()
    }

    pub fn text(&self) -> &'a str {
        todo!()
    }
}

#[derive(Debug)]
pub enum JsonErrorKind {}

#[derive(Debug)]
pub struct JsonError<'a> {
    pub kind: JsonErrorKind,
    pub context: Option<JsonErrorContext<'a>>,
    pub reason: Option<Box<dyn Send + Sync + std::error::Error>>,
}

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

#[derive(Debug)]
pub struct JsonParser<'a> {
    text: &'a str,
    remaining_text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonParser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            remaining_text: text.trim_start_matches(WHITESPACE_PATTERN),
            values: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<JsonValueStr, JsonError> {
        if let Some(root) = self.values.first() {
            self.remaining_text =
                &self.text[root.text.end..].trim_start_matches(WHITESPACE_PATTERN);
            self.values.clear();
        }
        todo!()
    }

    // TODO: expect_eos()

    pub fn is_eos(&self) -> bool {
        self.remaining_text.is_empty()
    }

    pub fn remaining_text(&self) -> &'a text {
        self.remaining_text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonValueStr<'a> {
    json: &'a JsonParser<'a>,
    index: usize,
}

impl<'a> JsonValueStr<'a> {
    pub fn kind(self) -> JsonValueStrKind {
        self.json.values[self.index].kind
    }

    pub fn text(self) -> &'a str {
        let text = &self.json.values[self.index].text;
        &self.json.text[text.start..text.end]
    }

    pub fn to_str(self) -> Cow<'a, str> {
        todo!()
    }

    // TODO: position() -> (NonZeroUsize, NonZeroUsize)
    // TODO: parent() -> Option<JsonValueStr>

    pub fn nullable<F, T, E>(&self, f: F) -> Result<Option<T>, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        (self.kind() != JsonValueStrKind::Null).then(f).transpose()
    }

    pub fn parse<T>(&self) -> Result<T, JsonError>
    where
        T: FromStr,
        T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        self.parse_with(|text| text.parse())
    }

    pub fn parse_with<F, T, E>(&self, f: F) -> Result<T, JsonError>
    where
        F: FnOnce(&str) -> Result<T, E>,
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        f(&self.to_str()).map_err(|_e| todo!())
    }

    pub fn integer(self) -> Result<Self, JsonError> {
        if !matches!(self.kind(), JsonValueStrKind::Number { integer: true }) {
            todo!();
        }
        Ok(self)
    }

    pub fn maybe_integer(self) -> Option<Self> {
        matches!(self.kind(), JsonValueStrKind::Number { integer: true }).then_some(self)
    }

    pub fn array(&self) -> Result<JsonArrayStr, JsonError> {
        todo!()
    }

    pub fn object(&self) -> Result<JsonObjectStr, JsonError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct JsonArrayStr<'a> {
    _value: JsonValueStr<'a>,
}

impl<'a> JsonArrayStr<'a> {
    pub fn get(&self, _index: usize) -> Option<JsonValueStr<'a>> {
        todo!()
    }

    pub fn expect(&self, _index: usize) -> Result<JsonValueStr<'a>, JsonError> {
        todo!()
    }
}

impl<'a> Iterator for JsonArrayStr<'a> {
    type Item = JsonValueStr<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug)]
pub struct JsonObjectStr<'a> {
    _value: JsonValueStr<'a>,
}

impl<'a> JsonObjectStr<'a> {
    pub fn expect(&self, _name: &str) -> Result<JsonValueStr<'a>, JsonError> {
        todo!()
    }
}

impl<'a> Iterator for JsonObjectStr<'a> {
    type Item = (JsonValueStr<'a>, JsonValueStr<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
