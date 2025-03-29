use std::{borrow::Cow, hash::Hash, num::NonZeroUsize, ops::Range, str::FromStr};

const WHITESPACE_PATTERN: [char; 4] = [' ', '\t', '\r', '\n'];
const NUMBER_START_PATTERN: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-'];
const DIGIT_PATTERN: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
#[non_exhaustive]
pub enum JsonError {
    UnexpectedEos {
        position: usize,
    },
    // TODO: rename
    NotEos {
        position: usize,
    },
    InvalidValue {
        position: usize,
    },
    InvalidNumber {
        position: usize,
    },
    InvalidString {
        position: usize,
    },
    Other {
        position: usize,
        source: Box<dyn Send + Sync + std::error::Error>,
    },
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
        parser.check_eos()?;
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
        } else if self.text.starts_with(NUMBER_START_PATTERN) {
            self.parse_number()?;
        } else if let Some(s) = self.text.strip_prefix('"') {
            self.parse_string(s)?;
        } else if !self.text.is_empty() {
            if self.text.starts_with(['+', '.']) {
                return Err(self.invalid_number());
            } else {
                let position = self.position();
                return Err(JsonError::InvalidValue { position });
            }
        } else {
            return Err(self.unexpected_eos());
        }
        Ok(())
    }

    // number = [ minus ] int [ frac ] [ exp ]
    fn parse_number(&mut self) -> Result<(), JsonError> {
        let mut kind = JsonValueStrKind::Number { integer: true };

        // [ minus ]
        let s = self.text.strip_prefix('-').unwrap_or(self.text);
        if s.is_empty() {
            return Err(self.unexpected_eos());
        }

        // int
        let s = if let Some(s) = s.strip_prefix('0') {
            if s.starts_with(DIGIT_PATTERN) {
                return Err(self.invalid_number());
            }
            s
        } else {
            s.strip_prefix(DIGIT_PATTERN)
                .ok_or_else(|| self.eos_or_number_error(s.is_empty()))
                .map(|s| s.trim_start_matches(DIGIT_PATTERN))?
        };

        // [ frac ]
        let s = if let Some(s) = s.strip_prefix('.') {
            kind = JsonValueStrKind::Number { integer: false };
            s.strip_prefix(DIGIT_PATTERN)
                .ok_or_else(|| self.eos_or_number_error(s.is_empty()))
                .map(|s| s.trim_start_matches(DIGIT_PATTERN))?
        } else {
            s
        };

        // [ exp ]
        let s = if let Some(s) = s.strip_prefix(['e', 'E']) {
            kind = JsonValueStrKind::Number { integer: false };
            let s = s.strip_prefix(['-', '+']).unwrap_or(s);
            s.strip_prefix(DIGIT_PATTERN)
                .ok_or_else(|| self.eos_or_number_error(s.is_empty()))
                .map(|s| s.trim_start_matches(DIGIT_PATTERN))?
        } else {
            s
        };

        if !(s.is_empty() || s.starts_with(WHITESPACE_PATTERN)) {
            return Err(self.invalid_number());
        }

        self.push_value(kind, self.text.len() - s.len());
        Ok(())
    }

    fn parse_string(&mut self, mut s: &'a str) -> Result<(), JsonError> {
        let mut kind = JsonValueStrKind::String { escaped: false };

        loop {
            s = s.trim_start_matches(|c| !(matches!(c, '"' | '\\') || c.is_ascii_control()));
            if let Some(s) = s.strip_prefix('"') {
                self.push_value(kind, self.text.len() - s.len());
                return Ok(());
            }
            if s.is_empty() {
                return Err(self.unexpected_eos());
            }

            kind = JsonValueStrKind::String { escaped: true };
            s = s.strip_prefix('\\').ok_or_else(|| self.invalid_string())?;
            match s.chars().next().ok_or_else(|| self.unexpected_eos())? {
                '"' | '\\' | '/' | '\n' | '\t' | '\r' | 'b' | 'f' => s = &s[1..],
                'u' => {
                    if s.len() < 5 {
                        return Err(self.unexpected_eos());
                    }
                    s.get(1..5)
                        .and_then(|code| u32::from_str_radix(code, 16).ok())
                        .and_then(char::from_u32)
                        .ok_or_else(|| self.invalid_string())?;
                    s = &s[5..];
                }
                _ => return Err(self.invalid_string()),
            }
        }
    }

    fn eos_or_number_error(&mut self, eos: bool) -> JsonError {
        if eos {
            self.unexpected_eos()
        } else {
            self.invalid_number()
        }
    }

    fn invalid_number(&self) -> JsonError {
        JsonError::InvalidNumber {
            position: self.position(),
        }
    }

    fn invalid_string(&self) -> JsonError {
        JsonError::InvalidString {
            position: self.position(),
        }
    }

    pub fn check_eos(&mut self) -> Result<(), JsonError> {
        self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
        if !self.text.is_empty() {
            return Err(JsonError::NotEos {
                position: self.position(),
            });
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

    fn unexpected_eos(&mut self) -> JsonError {
        self.text = &self.text[self.text.len()..];
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
        let json = JsonStr::parse(" null ")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueStrKind::Null);
        assert_eq!(value.text(), "null");
        assert_eq!(value.position(), 1);

        assert!(matches!(
            JsonStr::parse("nul"),
            Err(JsonError::InvalidValue { position: 0 })
        ));
        assert!(matches!(
            JsonStr::parse("nulla"),
            Err(JsonError::NotEos { position: 4 })
        ));

        Ok(())
    }

    #[test]
    fn parse_bool() -> Result<(), JsonError> {
        let json = JsonStr::parse("true")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueStrKind::Bool);
        assert_eq!(value.text(), "true");
        assert_eq!(value.position(), 0);

        let json = JsonStr::parse(" false ")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueStrKind::Bool);
        assert_eq!(value.text(), "false");
        assert_eq!(value.position(), 1);

        assert!(matches!(
            JsonStr::parse("false true"),
            Err(JsonError::NotEos { position: 6 })
        ));

        Ok(())
    }

    #[test]
    fn parse_number() -> Result<(), JsonError> {
        // Integers.
        for text in ["0", "-12"] {
            let json = JsonStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueStrKind::Number { integer: true });
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Floats.
        for text in ["12.3", "12.3e4", "12.3e-4", "-0.3e+4", "12E034"] {
            let json = JsonStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueStrKind::Number { integer: false });
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Invalid nubmers.
        for text in [
            "--1", "+2", "0123", "00", ".123", "1..2", "1ee2", "1e+-3", "123.4.5",
        ] {
            assert!(
                matches!(
                    JsonStr::parse(text),
                    Err(JsonError::InvalidNumber { position: 0 })
                ),
                "text={text}, error={:?}",
                JsonStr::parse(text)
            );
        }

        // Invalid values.
        for text in ["e123"] {
            assert!(
                matches!(
                    JsonStr::parse(text),
                    Err(JsonError::InvalidValue { position: 0 })
                ),
                "text={text}, error={:?}",
                JsonStr::parse(text)
            );
        }

        // Unexpected EOS.
        for text in ["123.", "-", "123e", "123e-"] {
            assert!(
                matches!(JsonStr::parse(text), Err(JsonError::UnexpectedEos { .. })),
                "text={text}, error={:?}",
                JsonStr::parse(text)
            );
        }

        Ok(())
    }
}
