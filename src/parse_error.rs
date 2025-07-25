use std::num::NonZeroUsize;

#[cfg(doc)]
use crate::RawJson;
use crate::{JsonValueKind, RawJsonValue};

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
    /// Makes a [`JsonParseError::InvalidValue`] error.
    pub fn invalid_value<E>(value: RawJsonValue<'_, '_>, error: E) -> JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        JsonParseError::InvalidValue {
            kind: value.kind(),
            position: value.position(),
            error: error.into(),
        }
    }

    /// Returns the kind of JSON value associated with the error.
    pub fn kind(&self) -> Option<JsonValueKind> {
        match self {
            JsonParseError::UnexpectedEos { kind, .. } => *kind,
            JsonParseError::UnexpectedTrailingChar { kind, .. } => Some(*kind),
            JsonParseError::UnexpectedValueChar { kind, .. } => *kind,
            JsonParseError::InvalidValue { kind, .. } => Some(*kind),
        }
    }

    /// Returns the byte position in the input string where the error occurred.
    pub fn position(&self) -> usize {
        match self {
            JsonParseError::UnexpectedEos { position, .. }
            | JsonParseError::UnexpectedTrailingChar { position, .. }
            | JsonParseError::UnexpectedValueChar { position, .. }
            | JsonParseError::InvalidValue { position, .. } => *position,
        }
    }

    /// Returns the line and column numbers for the error position in the input text.
    ///
    /// This method calculates the line and column numbers based on the error's
    /// position within the provided text. This is useful for creating human-readable
    /// error messages that can pinpoint the exact location of the error.
    ///
    /// Returns `None` if the position is outside the text boundaries or falls on an
    /// invalid UTF-8 boundary.
    ///
    /// # Note
    ///
    /// The column value counts each character as 1 column, regardless of its
    /// actual display width. For accurate display width calculation that accounts
    /// for multi-width characters (like CJK characters or emoji), consider using
    /// an external crate such as [`unicode-width`](https://crates.io/crates/unicode-width).
    pub fn get_line_and_column_numbers(&self, text: &str) -> Option<(NonZeroUsize, NonZeroUsize)> {
        let position = self.position();

        // Check if position is within bounds
        if position > text.len() {
            return None;
        }

        // If position is at the end of text, we need to handle it specially
        if position == text.len() {
            let mut line = 0;
            let mut column = 0;
            for c in text.chars() {
                if c == '\n' {
                    column = 0;
                    line += 1;
                } else {
                    column += 1;
                }
            }
            let line = NonZeroUsize::MIN.saturating_add(line);
            let column = NonZeroUsize::MIN.saturating_add(column);
            return Some((line, column));
        }

        // Check if position is on a valid UTF-8 boundary
        if !text.is_char_boundary(position) {
            return None;
        }

        let mut line = 0;
        let mut column = 0;
        for (i, c) in text.char_indices() {
            if i == position {
                let line = NonZeroUsize::MIN.saturating_add(line);
                let column = NonZeroUsize::MIN.saturating_add(column);
                return Some((line, column));
            }

            if c == '\n' {
                column = 0;
                line += 1;
            } else {
                // [NOTE]
                // This counts each character as 1 column, regardless of display width.
                // Multi-width characters (e.g., CJK, emoji) will be counted as 1 column.
                column += 1;
            }
        }

        // This should not be reached given our bounds check above
        None
    }

    /// Returns the line of text where the error occurred.
    ///
    /// This method extracts the entire line from the input text that contains the error.
    /// This is useful for error reporting as it provides context around the error location.
    ///
    /// Returns `None` if the position is outside the text boundaries.
    pub fn get_line<'a>(&self, text: &'a str) -> Option<&'a str> {
        let position = self.position();
        if !text.is_char_boundary(position) {
            return None;
        }

        let start = text[..position].rfind('\n').map(|i| i + 1).unwrap_or(0);
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
                    write!(
                        f,
                        "unexpected EOS while parsing {kind:?} at byte position {position}"
                    )
                } else {
                    write!(f, "unexpected EOS at byte position {position}")
                }
            }
            JsonParseError::UnexpectedTrailingChar { kind, position } => {
                write!(
                    f,
                    "unexpected trailing char after parsing {kind:?} at byte position {position}"
                )
            }
            JsonParseError::UnexpectedValueChar { kind, position } => {
                if let Some(kind) = kind {
                    write!(
                        f,
                        "unexpected char while parsing {kind:?} at byte position {position}"
                    )
                } else {
                    write!(f, "unexpected char at byte position {position}")
                }
            }
            JsonParseError::InvalidValue {
                kind,
                position,
                error,
            } => {
                write!(
                    f,
                    "JSON {kind:?} at byte position {position} is invalid: {error}"
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
