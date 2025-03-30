use crate::JsonValueKind;

/// JSON parse error.
#[derive(Debug)]
#[non_exhaustive]
pub enum JsonParseError {
    UnexpectedEos {
        kind: Option<JsonValueKind>,
        position: usize,
    },
    UnexpectedTrailingChar {
        kind: JsonValueKind,
        position: usize,
    },
    UnexpectedValueChar {
        kind: Option<JsonValueKind>,
        position: usize,
    },
    /// Proper JSON value, but the content is invalid.
    InvalidValue {
        kind: JsonValueKind,
        position: usize,
        error: Box<dyn Send + Sync + std::error::Error>,
    },
    // TODO: remove
    UnexpectedKind {
        expected_kinds: &'static [JsonValueKind],
        actual_kind: JsonValueKind,
        position: usize, // TODO: range
    },
    UnexpectedArraySize {
        expected: usize,
        actual: usize,
        position: usize,
    },
    MissingRequiredMember {
        member_names: Vec<String>,
        position: usize,
    },
}

impl JsonParseError {
    pub fn position(&self) -> usize {
        match self {
            JsonParseError::UnexpectedEos { position, .. }
            | JsonParseError::UnexpectedTrailingChar { position, .. }
            | JsonParseError::UnexpectedValueChar { position, .. }
            | JsonParseError::UnexpectedKind { position, .. }
            | JsonParseError::InvalidValue { position, .. }
            | JsonParseError::UnexpectedArraySize { position, .. }
            | JsonParseError::MissingRequiredMember { position, .. } => *position,
        }
    }

    // TODO: row_column_line()
    // TDOO: get_value()
}
