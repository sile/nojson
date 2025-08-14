use std::ops::Range;

use crate::{
    JsonValueKind,
    raw::{JsonParseError, JsonValueIndexEntry},
};

const WHITESPACE_PATTERN: [char; 4] = [' ', '\t', '\r', '\n'];

pub trait HandleComment {
    fn handle_comment<'a>(&mut self, original_text: &'a str, text: &'a str) -> Option<&'a str>;
}

#[derive(Debug)]
pub struct NoopCommentHandler;

impl HandleComment for NoopCommentHandler {
    fn handle_comment<'a>(&mut self, _original_text: &'a str, text: &'a str) -> Option<&'a str> {
        Some(text)
    }
}

#[derive(Debug, Default)]
pub struct JsoncCommentHandler {
    pub comments: Vec<Range<usize>>,
}

impl HandleComment for JsoncCommentHandler {
    fn handle_comment<'a>(&mut self, original_text: &'a str, mut text: &'a str) -> Option<&'a str> {
        loop {
            let start = original_text.len() - text.len();

            text = if let Some(text) = text.strip_prefix("//") {
                text.trim_start_matches(|c| c != '\n')
            } else if let Some(text) = text.strip_prefix("/*") {
                let offset = text.find("*/")?;
                &text[offset + 2..]
            } else {
                break;
            };

            let end = original_text.len() - text.len();
            self.comments.push(Range { start, end });
            text = text.trim_start_matches(WHITESPACE_PATTERN);
        }
        Some(text)
    }
}

#[derive(Debug)]
pub struct JsonParser<'a, H> {
    original_text: &'a str,
    text: &'a str,
    kind: Option<JsonValueKind>,
    values: Vec<JsonValueIndexEntry>,
    handler: H,
}

impl<'a, H: HandleComment> JsonParser<'a, H> {
    pub fn new(text: &'a str, handler: H) -> Self {
        Self {
            original_text: text,
            text,
            kind: None,
            values: Vec::new(),
            handler,
        }
    }

    pub fn parse(mut self) -> Result<(Vec<JsonValueIndexEntry>, H), JsonParseError> {
        self.parse_value()?;
        self.check_trailing_char()?;
        Ok((self.values, self.handler))
    }

    fn check_trailing_char(&mut self) -> Result<(), JsonParseError> {
        self.text = self.skip_whitespaces_and_comments(self.text)?;
        if !self.text.is_empty() {
            return Err(JsonParseError::UnexpectedTrailingChar {
                kind: self.kind.expect("infallible"),
                position: self.position(),
            });
        }
        Ok(())
    }

    fn skip_whitespaces_and_comments(&mut self, s: &'a str) -> Result<&'a str, JsonParseError> {
        let s = s.trim_start_matches(WHITESPACE_PATTERN);
        if let Some(s) = self.handler.handle_comment(self.original_text, s) {
            Ok(s)
        } else {
            Err(self.unexpected_eos())
        }
    }

    fn parse_value(&mut self) -> Result<(), JsonParseError> {
        self.text = self.skip_whitespaces_and_comments(self.text)?;
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
        self.parse_literal(JsonValueKind::Boolean, "rue", s)
    }

    fn parse_false(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.parse_literal(JsonValueKind::Boolean, "alse", s)
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

    fn strip_char(&self, s: &'a str, c: char) -> Result<&'a str, JsonParseError> {
        s.strip_prefix(c)
            .ok_or_else(|| self.unexpected_value_char(self.offset(s)))
    }

    fn strip_one_or_more_digits(&self, s: &'a str) -> Result<&'a str, JsonParseError> {
        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        s.strip_prefix(digits)
            .ok_or_else(|| self.unexpected_value_char(self.offset(s)))
            .map(|s| s.trim_start_matches(digits))
    }

    fn parse_object(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.kind = Some(JsonValueKind::Object);

        let s = self.skip_whitespaces_and_comments(s)?;
        if let Some(s) = s.strip_prefix('}') {
            self.push_entry(self.offset(s));
            return Ok(());
        }

        let index = self.values.len();
        self.push_entry(self.offset(s)); // Push a placeholder entry
        self.text = s;

        loop {
            // Key.
            let s = self.strip_char(self.text, '"')?;
            self.parse_string(s)?;
            self.kind = Some(JsonValueKind::Object);

            // Value.
            self.text = self.skip_whitespaces_and_comments(self.text)?;
            self.text = self.strip_char(self.text, ':')?;
            self.parse_value()?;
            self.kind = Some(JsonValueKind::Object);

            self.text = self.skip_whitespaces_and_comments(self.text)?;
            if let Some(s) = self.text.strip_prefix('}') {
                self.text = s;
                self.finalize_entry(index);
                return Ok(());
            }

            self.text = self.strip_char(self.text, ',')?;
            self.text = self.skip_whitespaces_and_comments(self.text)?;
        }
    }

    fn parse_array(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        self.kind = Some(JsonValueKind::Array);

        let s = self.skip_whitespaces_and_comments(s)?;
        if let Some(s) = s.strip_prefix(']') {
            self.push_entry(self.offset(s));
            return Ok(());
        }

        let index = self.values.len();
        self.push_entry(self.offset(s)); // Push a placeholder entry

        loop {
            self.parse_value()?;
            self.kind = Some(JsonValueKind::Array);

            self.text = self.skip_whitespaces_and_comments(self.text)?;
            if let Some(s) = self.text.strip_prefix(']') {
                self.text = s;
                self.finalize_entry(index);
                return Ok(());
            } else {
                self.text = self.strip_char(self.text, ',')?;
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
            s = self.strip_char(s, '\\')?;
            if let Some(suffix) = s.strip_prefix(['"', '\\', '/', 'n', 't', 'r', 'b', 'f']) {
                s = suffix;
            } else {
                s = self.strip_char(s, 'u')?;
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
