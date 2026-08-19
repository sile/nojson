use alloc::vec::Vec;
use core::ops::Range;

use crate::{
    JsonValueKind,
    raw::{JsonParseError, JsonValueIndexEntry},
};

/// Maximum nesting depth of arrays and objects the parser accepts.
///
/// Inputs whose nesting would step past this limit are rejected with a
/// [`JsonParseError::InvalidValue`] whose message contains
/// `"nesting depth exceeded"` (the exact variant used to report the
/// rejection may be refined in future versions).
///
/// The numeric value may change in future versions; do not depend on its
/// exact value in `const` contexts.
pub const MAX_NESTING_DEPTH: usize = 128;

// If `MAX_NESTING_DEPTH` is changed, update the hard-coded `(128)` in
// `JsonParser::nesting_too_deep`'s `MSG` string in the same commit.
const _: () = assert!(
    MAX_NESTING_DEPTH == 128,
    "MAX_NESTING_DEPTH changed; update `nesting_too_deep`'s MSG to match",
);

pub trait Extensions {
    const ALLOW_COMMENTS: bool;
    const ALLOW_TRAILING_COMMAS: bool;
}

#[derive(Debug)]
pub struct Plain;

impl Extensions for Plain {
    const ALLOW_COMMENTS: bool = false;
    const ALLOW_TRAILING_COMMAS: bool = false;
}

#[derive(Debug)]
pub struct Jsonc;

impl Extensions for Jsonc {
    const ALLOW_COMMENTS: bool = true;
    const ALLOW_TRAILING_COMMAS: bool = true;
}

#[derive(Debug)]
pub struct JsonParser<'a, X> {
    original_text: &'a str,
    text: &'a str,
    kind: Option<JsonValueKind>,
    values: Vec<JsonValueIndexEntry>,
    comments: Vec<Range<usize>>,
    depth: usize,
    _extensions: core::marker::PhantomData<X>,
}

impl<'a, E: Extensions> JsonParser<'a, E> {
    pub fn new(text: &'a str) -> Self {
        Self {
            original_text: text,
            text,
            kind: None,
            values: Vec::new(),
            comments: Vec::new(),
            depth: 0,
            _extensions: core::marker::PhantomData,
        }
    }

    pub fn parse(
        mut self,
    ) -> Result<(Vec<JsonValueIndexEntry>, Vec<Range<usize>>), JsonParseError> {
        self.parse_value()?;
        self.check_trailing_char()?;
        Ok((self.values, self.comments))
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
        let mut s = &s[crate::swar::skip_json_whitespace(s.as_bytes())..];
        if !E::ALLOW_COMMENTS {
            return Ok(s);
        }

        loop {
            let start = self.original_text.len() - s.len();

            s = if let Some(s) = s.strip_prefix("//") {
                s.trim_start_matches(|c| c != '\n')
            } else if let Some(s) = s.strip_prefix("/*") {
                let Some(offset) = s.find("*/") else {
                    return Err(self.unexpected_eos());
                };
                &s[offset + 2..]
            } else {
                break;
            };

            let end = self.original_text.len() - s.len();
            self.comments.push(Range { start, end });
            s = &s[crate::swar::skip_json_whitespace(s.as_bytes())..];
        }
        Ok(s)
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
        let n = crate::swar::skip_ascii_digits(s.as_bytes());
        if n == 0 {
            Err(self.unexpected_value_char(self.offset(s)))
        } else {
            Ok(&s[n..])
        }
    }

    fn parse_object(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        if self.depth >= MAX_NESTING_DEPTH {
            return Err(self.nesting_too_deep(JsonValueKind::Object));
        }
        self.depth += 1;
        let result = self.parse_object_inner(s);
        self.depth -= 1;
        result
    }

    fn parse_object_inner(&mut self, s: &'a str) -> Result<(), JsonParseError> {
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
            if E::ALLOW_TRAILING_COMMAS
                && let Some(s) = self.text.strip_prefix('}')
            {
                self.text = s;
                self.finalize_entry(index);
                return Ok(());
            }
        }
    }

    fn parse_array(&mut self, s: &'a str) -> Result<(), JsonParseError> {
        if self.depth >= MAX_NESTING_DEPTH {
            return Err(self.nesting_too_deep(JsonValueKind::Array));
        }
        self.depth += 1;
        let result = self.parse_array_inner(s);
        self.depth -= 1;
        result
    }

    fn nesting_too_deep(&self, kind: JsonValueKind) -> JsonParseError {
        // The message is a `&'static str`, so no `format!` runs at error time.
        // `Box::<dyn Error>::from(&str)` internally still allocates twice
        // (`String::from` + boxed wrapper); avoiding that would require a
        // dedicated error type. The numeric value is locked to
        // `MAX_NESTING_DEPTH` by a compile-time assert below the const.
        const MSG: &str = "nesting depth exceeded MAX_NESTING_DEPTH (128)";
        JsonParseError::InvalidValue {
            kind,
            position: self.position(),
            error: MSG.into(),
        }
    }

    fn parse_array_inner(&mut self, s: &'a str) -> Result<(), JsonParseError> {
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

            if E::ALLOW_TRAILING_COMMAS {
                self.text = self.skip_whitespaces_and_comments(self.text)?;
                if let Some(s) = self.text.strip_prefix(']') {
                    self.text = s;
                    self.finalize_entry(index);
                    return Ok(());
                }
            }
        }
    }

    fn parse_string(&mut self, mut s: &'a str) -> Result<(), JsonParseError> {
        let mut escaped = false;
        self.kind = Some(JsonValueKind::String);

        loop {
            let skip = crate::swar::skip_plain_ascii_bytes(s.as_bytes());
            s = &s[skip..];
            let skip = crate::swar::skip_non_ascii_bytes(s.as_bytes());
            if skip > 0 {
                s = &s[skip..];
                continue;
            }
            match s.as_bytes().first().copied() {
                Some(b'"') => {
                    let s = &s[1..];
                    self.push_entry(self.offset(s));
                    self.values.last_mut().expect("infallible").escaped = escaped;
                    return Ok(());
                }
                Some(b'\\') => {
                    escaped = true;
                    s = &s[1..];
                    if let Some(suffix) = s.strip_prefix(['"', '\\', '/', 'n', 't', 'r', 'b', 'f'])
                    {
                        s = suffix;
                    } else {
                        s = self.strip_char(s, 'u')?;
                        if s.len() < 4 {
                            return Err(self.unexpected_eos());
                        }
                        let code = decode_hex_u16(s)
                            .ok_or_else(|| self.unexpected_value_char(self.offset(s)))?;
                        if (0xD800..=0xDBFF).contains(&code) {
                            // High surrogate must be followed by `\uXXXX`
                            // whose value is a low surrogate.
                            s = &s[4..];
                            if !s.starts_with("\\u") {
                                return Err(self.unexpected_value_char(self.offset(s)));
                            }
                            s = &s[2..];
                            if s.len() < 4 {
                                return Err(self.unexpected_eos());
                            }
                            let low = decode_hex_u16(s)
                                .ok_or_else(|| self.unexpected_value_char(self.offset(s)))?;
                            if !(0xDC00..=0xDFFF).contains(&low) {
                                return Err(self.unexpected_value_char(self.offset(s)));
                            }
                            s = &s[4..];
                        } else if (0xDC00..=0xDFFF).contains(&code) {
                            // Lone low surrogate is not allowed.
                            return Err(self.unexpected_value_char(self.offset(s)));
                        } else {
                            s = &s[4..];
                        }
                    }
                }
                Some(_) => {
                    return Err(self.unexpected_value_char(self.offset(s)));
                }
                None => {
                    return Err(self.unexpected_eos());
                }
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

#[inline(always)]
fn decode_hex_u16(s: &str) -> Option<u16> {
    let bytes = s.as_bytes().get(..4)?;
    let mut code = 0u16;
    for &byte in bytes {
        code = (code << 4) | decode_hex_nibble(byte)?;
    }
    Some(code)
}

#[inline(always)]
fn decode_hex_nibble(byte: u8) -> Option<u16> {
    match byte {
        b'0'..=b'9' => Some((byte - b'0') as u16),
        b'a'..=b'f' => Some((byte - b'a' + 10) as u16),
        b'A'..=b'F' => Some((byte - b'A' + 10) as u16),
        _ => None,
    }
}
