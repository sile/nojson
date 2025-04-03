use std::num::NonZeroUsize;

use crate::JsonValueKind;
#[cfg(doc)]
use crate::RawJson;

/// JSON parse error.
///
/// - [`JsonParseError::get_line()`]
/// - [`JsonParseError::get_line_and_column_numbers()`]
/// - [`RawJson::get_value_by_position()`]
#[derive(Debug)]
pub enum JsonParseError {
    /// End of string was reached unexpectedly while parsing a JSON value.
    UnexpectedEos {
        /// Expected kind of JSON value (if known).
        kind: Option<JsonValueKind>,

        /// Byte position in the input string where the error occurred.
        position: usize,
    },

    /// Additional characters were found after a complete JSON value was parsed.
    ///
    /// If it is intentional, you can re-parse the input by slicing the tail by using `position`.
    UnexpectedTrailingChar {
        /// Kind of JSON value that was successfully parsed
        kind: JsonValueKind,

        /// Byte position in the input string where the error occurred.
        position: usize,
    },

    /// An unexpected character was encountered while parsing a JSON value.
    UnexpectedValueChar {
        /// Expected kind of JSON value (if known).
        kind: Option<JsonValueKind>,

        /// Byte position in the input string where the error occurred.
        position: usize,
    },

    /// A JSON value was syntaxtically correct, but invalid according to its application specific format rules.
    InvalidValue {
        /// Kind of JSON value.
        kind: JsonValueKind,

        /// Byte position in the input string where the JSON value starts.
        position: usize,

        /// Concrete error reason.
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
