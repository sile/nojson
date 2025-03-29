use std::{borrow::Cow, hash::Hash, num::NonZeroUsize, ops::Range, str::FromStr};

const WHITESPACE_PATTERN: [char; 4] = [' ', '\t', '\r', '\n'];

#[derive(Debug)]
pub enum JsonError {
    UnexpectedEos { position: usize },
    NotValueStart { position: usize },
    //
    // pub kind: JsonErrorKind,
    // pub position: usize,
    // pub reason: Option<Box<dyn Send + Sync + std::error::Error>>,
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
pub struct JsonStr<'a> {
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonStr<'a> {
    pub fn parse(text: &'a str) -> Result<Self, JsonError> {
        let mut parser = JsonParser::new(text);
        parser.parse_value()?;
        Ok(Self {
            text,
            values: parser.values,
        })
    }

    pub fn value(&self) -> JsonValueStr {
        JsonValueStr {
            json: self,
            index: 0,
        }
    }

    pub fn remaining_text(&self) -> &'a str {
        &self.text[self.values[0].text.end..]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JsonValueStr<'a> {
    json: &'a JsonStr<'a>,
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

    pub fn position(self) -> usize {
        self.json.values[self.index].text.start
    }

    pub fn to_str(self) -> Cow<'a, str> {
        todo!()
    }

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

#[derive(Debug)]
struct JsonParser<'a> {
    original_text: &'a str,
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonParser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            original_text: text,
            text,
            values: Vec::new(),
        }
    }

    fn parse_value(&mut self) -> Result<(), JsonError> {
        self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
        if self.text.starts_with("null") {
            self.push_value(JsonValueStrKind::Null, "null".len());
        } else if self.text.starts_with("true") {
            self.push_value(JsonValueStrKind::Bool, "true".len());
        } else if self.text.starts_with("false") {
            self.push_value(JsonValueStrKind::Bool, "false".len());
        } else if !self.text.is_empty() {
            let position = self.position();
            return Err(JsonError::NotValueStart { position });
        } else {
            return Err(self.unexpected_eos());
        }
        Ok(())
    }

    fn push_value(&mut self, kind: JsonValueStrKind, len: usize) {
        let position = self.position();
        let entry = JsonValueIndexEntry {
            kind,
            text: Range {
                start: position,
                end: position + len,
            },
            size: NonZeroUsize::MIN,
        };
        self.values.push(entry);
        self.text = &self.text[len..];
    }

    fn position(&self) -> usize {
        self.original_text.len() - self.text.len()
    }

    fn unexpected_eos(&self) -> JsonError {
        JsonError::UnexpectedEos {
            position: self.position(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_text() {
        assert!(matches!(
            JsonStr::parse(""),
            Err(JsonError::UnexpectedEos { position: 0 })
        ));
        assert!(matches!(
            JsonStr::parse("    "),
            Err(JsonError::UnexpectedEos { position: 4 })
        ));
    }

    #[test]
    fn parse_null() -> Result<(), JsonError> {
        let json = JsonStr::parse(" null foo")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueStrKind::Null);
        assert_eq!(value.text(), "null");
        assert_eq!(value.position(), 1);
        assert_eq!(json.remaining_text(), " foo");

        assert!(matches!(
            JsonStr::parse("nul"),
            Err(JsonError::NotValueStart { position: 0 })
        ));

        Ok(())
    }

    #[test]
    fn parse_bool() -> Result<(), JsonError> {
        let json = JsonStr::parse("true false foo")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueStrKind::Bool);
        assert_eq!(value.text(), "true");
        assert_eq!(value.position(), 0);
        assert_eq!(json.remaining_text(), " false foo");

        let json = JsonStr::parse(json.remaining_text())?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueStrKind::Bool);
        assert_eq!(value.text(), "false");
        assert_eq!(value.position(), 1);
        assert_eq!(json.remaining_text(), " foo");

        assert!(matches!(
            JsonStr::parse(json.remaining_text()),
            Err(JsonError::NotValueStart { position: 1 })
        ));

        Ok(())
    }
}
