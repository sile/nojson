use std::ops::Range;

use crate::{
    JsonValueKind,
    str::{JsonParseError, JsonValueIndexEntry},
};

const WHITESPACE_PATTERN: [char; 4] = [' ', '\t', '\r', '\n'];
const DIGIT_PATTERN: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
pub(crate) struct JsonParser<'a> {
    original_text: &'a str,
    text: &'a str,
    kind: Option<JsonValueKind>,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonParser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            original_text: text,
            text,
            kind: None,
            values: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Vec<JsonValueIndexEntry>, JsonParseError> {
        self.parse_value()?;

        self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
        if !self.text.is_empty() {
            return Err(JsonParseError::UnexpectedTrailingChar {
                kind: self.kind.expect("infallible"),
                position: self.position(),
            });
        }

        Ok(self.values)
    }

    fn parse_value(&mut self) -> Result<(), JsonParseError> {
        self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
        match self.text.chars().next() {
            Some('n') => self.parse_null(&self.text[1..]),
            Some('t') => self.parse_true(&self.text[1..]),
            Some('f') => self.parse_false(&self.text[1..]),
            Some('"') => self.parse_string(&self.text[1..]),
            Some('[') => self.parse_array(&self.text[1..]),
            Some('{') => self.parse_object(&self.text[1..]),
            Some('0'..='9' | '-') => self.parse_number(),
            Some(_) => Err(self.unexpected_value_char(0)),
            None => Err(self.unexpected_eos()),
        }
    }

    fn parse_null(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.parse_literal(JsonValueKind::Null, "ull", s)
    }

    fn parse_true(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.parse_literal(JsonValueKind::Bool, "rue", s)
    }

    fn parse_false(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.parse_literal(JsonValueKind::Bool, "alse", s)
    }

    fn parse_literal(
        &mut self,
        kind: JsonValueKind,
        literal_suffix: &str,
        s: &'a str,
    ) -> Result<(), JsonParseError> {
        self.kind = Some(kind);
        if s.starts_with(literal_suffix) {
            self.push_entry(1 + literal_suffix.len());
            Ok(())
        } else {
            for (i, (c0, c1)) in s.chars().zip(literal_suffix.chars()).enumerate() {
                if c0 != c1 {
                    return Err(self.unexpected_value_char(1 + i));
                }
            }
            Err(self.unexpected_eos())
        }
    }

    fn unexpected_value_char(&self, offset: usize) -> JsonParseError {
        let kind = self.kind;
        let position = self.position() + offset;
        if position == self.original_text.len() {
            JsonParseError::UnexpectedEos { kind, position }
        } else {
            JsonParseError::UnexpectedValueChar { kind, position }
        }
    }

    // number = [ minus ] int [ frac ] [ exp ]
    fn parse_number(&mut self) -> Result<(), JsonParseError> {
        self.kind = Some(JsonValueKind::Integer);

        // [ minus ]
        let s = self.text.strip_prefix('-').unwrap_or(self.text);

        // int
        let s = if let Some(s) = s.strip_prefix('0') {
            s
        } else {
            self.strip_one_or_more_digits(s)?
        };

        // [ frac ]
        let s = if let Some(s) = s.strip_prefix('.') {
            self.kind = Some(JsonValueKind::Float);
            self.strip_one_or_more_digits(s)?
        } else {
            s
        };

        // [ exp ]
        let s = if let Some(s) = s.strip_prefix(['e', 'E']) {
            self.kind = Some(JsonValueKind::Float);
            let s = s.strip_prefix(['-', '+']).unwrap_or(s);
            self.strip_one_or_more_digits(s)?
        } else {
            s
        };

        self.push_entry(self.offset(s));
        Ok(())
    }

    fn strip_one_or_more_digits(&self, s: &'a str) -> Result<&'a str, JsonParseError> {
        s.strip_prefix(DIGIT_PATTERN)
            .ok_or_else(|| self.unexpected_value_char(self.offset(s)))
            .map(|s| s.trim_start_matches(DIGIT_PATTERN))
    }

    fn parse_object(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.kind = Some(JsonValueKind::Object);

        let s = s.trim_start_matches(WHITESPACE_PATTERN);
        if let Some(s) = s.strip_prefix('}') {
            self.push_entry(self.offset(s));
            return Ok(());
        }

        let index = self.values.len();
        self.push_entry(self.offset(s)); // Push a placeholder entry
        self.text = s;

        loop {
            // Key.
            let s = self
                .text
                .strip_prefix('"')
                .ok_or_else(|| self.unexpected_value_char(0))?;
            self.parse_string(s)?;
            self.kind = Some(JsonValueKind::Object);

            // Value.
            self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
            self.text = self
                .text
                .strip_prefix(':')
                .ok_or_else(|| self.unexpected_value_char(0))?;
            self.parse_value()?;
            self.kind = Some(JsonValueKind::Object);

            self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
            if let Some(s) = self.text.strip_prefix('}') {
                self.text = s;
                self.finalize_entry(index);
                return Ok(());
            }

            self.text = self
                .text
                .strip_prefix(',')
                .ok_or_else(|| self.unexpected_value_char(0))?;
            self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
        }
    }

    fn parse_array(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.kind = Some(JsonValueKind::Array);

        let s = s.trim_start_matches(WHITESPACE_PATTERN);
        if let Some(s) = s.strip_prefix(']') {
            self.push_entry(self.offset(s));
            return Ok(());
        }

        let index = self.values.len();
        self.push_entry(self.offset(s)); // Push a placeholder entry

        loop {
            self.parse_value()?;
            self.kind = Some(JsonValueKind::Array);

            let s = self.text.trim_start_matches(WHITESPACE_PATTERN);
            if let Some(s) = s.strip_prefix(']') {
                self.text = s;
                self.finalize_entry(index);
                return Ok(());
            } else if let Some(s) = s.strip_prefix(',') {
                self.text = s;
            } else {
                return Err(self.unexpected_value_char(self.offset(s)));
            }
        }
    }

    fn parse_string(&mut self, mut s: &'a str) -> Result<(), JsonParseError> {
        let mut escaped = false;
        self.kind = Some(JsonValueKind::String);

        loop {
            s = s.trim_start_matches(|c| !(matches!(c, '"' | '\\') || c.is_ascii_control()));
            if let Some(s) = s.strip_prefix('"') {
                self.push_entry(self.offset(s));
                self.values.last_mut().expect("infallible").escaped = escaped;
                return Ok(());
            }

            escaped = true;
            s = s
                .strip_prefix('\\')
                .ok_or_else(|| self.unexpected_value_char(self.offset(s)))?;
            if let Some(suffix) = s.strip_prefix(['"', '\\', '/', 'n', 't', 'r', 'b', 'f']) {
                s = suffix;
            } else {
                s = s
                    .strip_prefix('u')
                    .ok_or_else(|| self.unexpected_value_char(self.offset(s)))?;
                if s.len() < 4 {
                    return Err(self.unexpected_eos());
                }
                s.get(0..4)
                    .and_then(|code| u32::from_str_radix(code, 16).ok())
                    .and_then(char::from_u32)
                    .ok_or_else(|| self.unexpected_value_char(self.offset(s)))?;
                s = &s[4..];
            }
        }
    }

    fn push_entry(&mut self, len: usize) {
        let position = self.position();
        let entry = JsonValueIndexEntry {
            kind: self.kind.expect("infallible"),
            escaped: false,
            text: Range {
                start: position,
                end: position + len,
            },
            end_index: self.values.len() + 1,
        };
        self.values.push(entry);
        self.text = &self.text[len..];
    }

    fn finalize_entry(&mut self, index: usize) {
        self.values[index].text.end = self.position();
        self.values[index].end_index = self.values.len();
    }

    fn position(&self) -> usize {
        self.original_text.len() - self.text.len()
    }

    fn offset(&self, s: &str) -> usize {
        self.text.len() - s.len()
    }

    fn unexpected_eos(&self) -> JsonParseError {
        JsonParseError::UnexpectedEos {
            kind: self.kind,
            position: self.original_text.len(),
        }
    }
}
