use std::ops::Range;

use crate::{
    JsonValueKind,
    str::{JsonParseError, JsonValueIndexEntry},
};

const WHITESPACE_PATTERN: [char; 4] = [' ', '\t', '\r', '\n'];
const NUMBER_START_PATTERN: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-'];
const NUMBER_END_PATTERN: [char; 7] = [' ', '\t', '\r', '\n', ',', ']', '}'];
const DIGIT_PATTERN: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
pub(crate) struct JsonParser<'a> {
    original_text: &'a str,
    text: &'a str,
    pub values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonParser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            original_text: text,
            text,
            values: Vec::new(),
        }
    }

    pub fn parse_value(&mut self) -> Result<(), JsonParseError> {
        self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
        match self.text.chars().next() {
            Some('n') => self.parse_null(&self.text[1..]),
            Some('t') => self.parse_true(&self.text[1..]),
            Some('f') => self.parse_false(&self.text[1..]),
            Some('"') => self.parse_string(&self.text[1..]),
            Some('[') => self.parse_array(&self.text[1..]),
            Some('{') => self.parse_object(&self.text[1..]),
            None => Err(self.unexpected_eos()),
            _ => {
                if self.text.starts_with(NUMBER_START_PATTERN) {
                    self.parse_number()
                } else {
                    if self.text.starts_with(['+', '.']) {
                        Err(self.invalid_number())
                    } else if self.text.starts_with([']']) {
                        let position = self.position();
                        Err(JsonParseError::UnmatchedArrayClose { position })
                    } else if self.text.starts_with(['}']) {
                        let position = self.position();
                        Err(JsonParseError::UnmatchedObjectClose { position })
                    } else {
                        let position = self.position();
                        Err(JsonParseError::UnexpectedChar { position })
                    }
                }
            }
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
        if s.starts_with(literal_suffix) {
            self.push_value(kind, 1 + literal_suffix.len());
            Ok(())
        } else {
            for (i, (c0, c1)) in s.chars().zip(literal_suffix.chars()).enumerate() {
                if c0 != c1 {
                    return Err(self.malformed_value(kind, 1 + i));
                }
            }
            Err(self.unexpected_eos())
        }
    }

    fn malformed_value(&self, kind: JsonValueKind, len: usize) -> JsonParseError {
        JsonParseError::MalformedValue {
            kind,
            position_range: self.position_range(len),
        }
    }

    fn position_range(&self, len: usize) -> Range<usize> {
        let start = self.position();
        let end = start + len;
        Range { start, end }
    }

    // number = [ minus ] int [ frac ] [ exp ]
    fn parse_number(&mut self) -> Result<(), JsonParseError> {
        let mut kind = JsonValueKind::Integer;

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
            kind = JsonValueKind::Float;
            s.strip_prefix(DIGIT_PATTERN)
                .ok_or_else(|| self.eos_or_number_error(s.is_empty()))
                .map(|s| s.trim_start_matches(DIGIT_PATTERN))?
        } else {
            s
        };

        // [ exp ]
        let s = if let Some(s) = s.strip_prefix(['e', 'E']) {
            kind = JsonValueKind::Float;
            let s = s.strip_prefix(['-', '+']).unwrap_or(s);
            s.strip_prefix(DIGIT_PATTERN)
                .ok_or_else(|| self.eos_or_number_error(s.is_empty()))
                .map(|s| s.trim_start_matches(DIGIT_PATTERN))?
        } else {
            s
        };

        if !(s.is_empty() || s.starts_with(NUMBER_END_PATTERN)) {
            return Err(self.invalid_number());
        }

        self.push_value(kind, self.text.len() - s.len());
        Ok(())
    }

    fn parse_object(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        let s = s.trim_start_matches(WHITESPACE_PATTERN);
        if let Some(s) = s.strip_prefix('}') {
            self.push_value(JsonValueKind::Object, self.text.len() - s.len());
            return Ok(());
        }

        let index = self.values.len();
        self.push_value(JsonValueKind::Object, self.text.len() - s.len());
        self.text = s;

        loop {
            self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
            let s = self
                .text
                .strip_prefix('"')
                .ok_or_else(|| self.eos_or_invalid_object())?;
            self.parse_string(s)?;

            self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
            self.text = self
                .text
                .strip_prefix(':')
                .ok_or_else(|| self.eos_or_invalid_object())?;
            self.parse_value()?;

            self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
            if let Some(s) = self.text.strip_prefix('}') {
                self.text = s;
                self.values[index].text.end = self.position();
                self.values[index].end_index = self.values.len();
                return Ok(());
            }
            self.text = self
                .text
                .strip_prefix(',')
                .ok_or_else(|| self.eos_or_invalid_object())?;
        }
    }

    fn parse_array(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        let s = s.trim_start_matches(WHITESPACE_PATTERN);
        if let Some(s) = s.strip_prefix(']') {
            self.push_value(JsonValueKind::Array, self.text.len() - s.len());
            return Ok(());
        }

        let index = self.values.len();
        self.push_value(JsonValueKind::Array, self.text.len() - s.len());

        loop {
            self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
            if self.text.starts_with([',', ']']) {
                return Err(self.invalid_array());
            }

            self.parse_value()?;

            let s = self.text.trim_start_matches(WHITESPACE_PATTERN);
            if let Some(s) = s.strip_prefix(']') {
                self.text = s;
                self.values[index].text.end = self.position();
                self.values[index].end_index = self.values.len();
                return Ok(());
            } else if let Some(s) = s.strip_prefix(',') {
                self.text = s;
            } else if s.is_empty() {
                return Err(self.unexpected_eos());
            } else if s.starts_with(['}']) {
                self.text = s;
                let position = self.position();
                return Err(JsonParseError::UnmatchedObjectClose { position });
            } else {
                return Err(self.invalid_array());
            }
        }
    }

    fn parse_string(&mut self, mut s: &'a str) -> Result<(), JsonParseError> {
        let mut escaped = false;

        loop {
            s = s.trim_start_matches(|c| !(matches!(c, '"' | '\\') || c.is_ascii_control()));
            if let Some(s) = s.strip_prefix('"') {
                self.push_value(JsonValueKind::String, self.text.len() - s.len());
                self.values.last_mut().expect("infallible").escaped = escaped;
                return Ok(());
            }
            if s.is_empty() {
                return Err(self.unexpected_eos());
            }

            escaped = true;
            s = s.strip_prefix('\\').ok_or_else(|| self.invalid_string())?;
            match s.chars().next().ok_or_else(|| self.unexpected_eos())? {
                '"' | '\\' | '/' | 'n' | 't' | 'r' | 'b' | 'f' => s = &s[1..],
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

    fn eos_or_number_error(&mut self, eos: bool) -> JsonParseError {
        if eos {
            self.unexpected_eos()
        } else {
            self.invalid_number()
        }
    }

    // TODO: rename
    fn eos_or_invalid_object(&mut self) -> JsonParseError {
        if self.text.is_empty() {
            self.unexpected_eos()
        } else if self.text.starts_with(']') {
            let position = self.position();
            JsonParseError::UnmatchedArrayClose { position }
        } else {
            self.invalid_object()
        }
    }

    fn invalid_object(&self) -> JsonParseError {
        JsonParseError::InvalidObject {
            position: self.position(),
        }
    }

    fn invalid_array(&self) -> JsonParseError {
        JsonParseError::InvalidArray {
            position: self.position(),
        }
    }

    fn invalid_number(&self) -> JsonParseError {
        JsonParseError::InvalidNumber {
            position: self.position(),
        }
    }

    fn invalid_string(&self) -> JsonParseError {
        JsonParseError::InvalidString {
            position: self.position(),
        }
    }

    pub fn check_eos(&mut self) -> Result<(), JsonParseError> {
        self.text = self.text.trim_start_matches(WHITESPACE_PATTERN);
        if self.text.starts_with(']') {
            return Err(JsonParseError::UnmatchedArrayClose {
                position: self.position(),
            });
        } else if self.text.starts_with('}') {
            return Err(JsonParseError::UnmatchedObjectClose {
                position: self.position(),
            });
        } else if !self.text.is_empty() {
            return Err(JsonParseError::UnexpectedTrailingChars {
                position: self.position(),
            });
        }
        Ok(())
    }

    fn push_value(&mut self, kind: JsonValueKind, len: usize) {
        let position = self.position();
        let entry = JsonValueIndexEntry {
            kind,
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

    fn position(&self) -> usize {
        self.original_text.len() - self.text.len()
    }

    fn unexpected_eos(&mut self) -> JsonParseError {
        self.text = &self.text[self.text.len()..];
        JsonParseError::UnexpectedEos {
            position: self.position(),
        }
    }
}
