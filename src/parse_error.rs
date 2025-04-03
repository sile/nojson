use std::num::NonZeroUsize;

use crate::JsonValueKind;
#[cfg(doc)]
use crate::RawJson;

/// JSON parse error.
///
/// This enum represents various errors that can occur during JSON parsing.
/// Each variant provides specific details about the error, including the position
/// in the input string where the error occurred.
///
/// For generating more detailed error messages, you can use these methods:
/// - [`JsonParseError::get_line()`]
/// - [`JsonParseError::get_line_and_column_numbers()`]
/// - [`RawJson::get_value_by_position()`]
///
/// These methods help provide context for debugging and error reporting.
#[derive(Debug)]
pub enum JsonParseError {
    /// End of string was reached unexpectedly while parsing a JSON value.
    ///
    /// This occurs when the parser reaches the end of the input string
    /// before completing the parse of the current JSON value.
    /// For example, a string that starts with a quote but doesn't have a closing quote.
    UnexpectedEos {
        /// Kind of JSON value that was being parsed when the unexpected end was encountered.
        kind: Option<JsonValueKind>,

        /// Byte position in the input string where the unexpected end occurred.
        position: usize,
    },

    /// Additional non-whitespace characters were found after a complete JSON value was parsed.
    ///
    /// This happens when the input contains extra data after a valid JSON value.
    /// For example, `{"key": "value"} extra`.
    UnexpectedTrailingChar {
        /// Kind of JSON value that was successfully parsed before the trailing characters.
        kind: JsonValueKind,

        /// Byte position in the input string where the non-whitespace trailing characters begin.
        position: usize,
    },

    /// An unexpected character was encountered while parsing a JSON value.
    ///
    /// This occurs when the parser encounters a character that doesn't match
    /// the expected syntax for the current context. For example, encountering
    /// a letter when expecting a digit in a number, or an invalid escape sequence
    /// in a string.
    UnexpectedValueChar {
        /// Kind of JSON value that was being parsed when the unexpected character was encountered.
        kind: Option<JsonValueKind>,

        /// Byte position in the input string where the unexpected character was found.
        position: usize,
    },

    /// A JSON value was syntactically correct, but invalid according to application-specific format rules.
    ///
    /// This happens when the JSON syntax is valid, but the value doesn't conform to
    /// additional constraints. For example, an integer that exceeds the maximum
    /// allowed value, or a string that doesn't match an expected pattern or format.
    InvalidValue {
        /// Kind of JSON value that failed validation.
        kind: JsonValueKind,

        /// Byte position in the input string where the invalid JSON value starts.
        position: usize,

        /// Error reason that describes why the value is invalid.
        error: Box<dyn Send + Sync + std::error::Error>,
    },
}

impl JsonParseError {
    pub fn kind(&self) -> Option<JsonValueKind> {
        match self {
            JsonParseError::UnexpectedEos { kind, .. } => *kind,
            JsonParseError::UnexpectedTrailingChar { kind, .. } => Some(*kind),
            JsonParseError::UnexpectedValueChar { kind, .. } => *kind,
            JsonParseError::InvalidValue { kind, .. } => Some(*kind),
        }
    }

    pub fn position(&self) -> usize {
        match self {
            JsonParseError::UnexpectedEos { position, .. }
            | JsonParseError::UnexpectedTrailingChar { position, .. }
            | JsonParseError::UnexpectedValueChar { position, .. }
            | JsonParseError::InvalidValue { position, .. } => *position,
        }
    }

    pub fn get_line_and_column_numbers(&self, text: &str) -> Option<(NonZeroUsize, NonZeroUsize)> {
        let position = self.position();
        let mut line = 0;
        let mut column = 0;
        for (i, c) in text.char_indices().take_while(|(i, _)| *i <= position) {
            if i == position {
                let line = NonZeroUsize::MIN.saturating_add(line);
                let column = NonZeroUsize::MIN.saturating_add(column);
                return Some((line, column));
            }

            if c == '\n' {
                column = 0;
                line += 1;
            } else {
                // [NOTE] Multi-byte chars are not taken into account.
                column += 1;
            }
        }
        None
    }

    pub fn get_line<'a>(&self, text: &'a str) -> Option<&'a str> {
        let position = self.position();
        if !text.is_char_boundary(position) {
            return None;
        }

        let start = text[..position]
            .rfind('\n')
            .map(|i| position - i)
            .unwrap_or(0);
        let end = text[position..]
            .find('\n')
            .map(|i| position + i)
            .unwrap_or_else(|| text.len());
        Some(&text[start..end])
    }
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonParseError::UnexpectedEos { kind, position } => {
                if let Some(kind) = kind {
                    let kind = kind.name();
                    write!(
                        f,
                        "unexpected EOS while parsing {} at byte position {}",
                        kind, position
                    )
                } else {
                    write!(f, "unexpected EOS at byte position {}", position)
                }
            }
            JsonParseError::UnexpectedTrailingChar { kind, position } => {
                let kind = kind.name();
                write!(
                    f,
                    "unexpected trailing char after parsing {} at byte position {}",
                    kind, position
                )
            }
            JsonParseError::UnexpectedValueChar { kind, position } => {
                if let Some(kind) = kind {
                    let kind = kind.name();
                    write!(
                        f,
                        "unexpected char while parsing {} at byte position {}",
                        kind, position
                    )
                } else {
                    write!(f, "unexpected char at byte position {}", position)
                }
            }
            JsonParseError::InvalidValue {
                kind,
                position,
                error,
            } => {
                let kind = kind.name();
                write!(
                    f,
                    "JSON {} at byte position {} is invalid: {}",
                    kind, position, error
                )
            }
        }
    }
}

impl std::error::Error for JsonParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::InvalidValue { error, .. } = self {
            Some(&**error)
        } else {
            None
        }
    }
}
