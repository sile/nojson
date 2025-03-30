use crate::JsonValueKind;

/// JSON parse error.
#[derive(Debug)]
#[non_exhaustive]
pub enum JsonParseError {
    UnexpectedEos {
        position: usize,
    },
    UnexpectedLeadingChar {
        position: usize,
    },
    UnexpectedTrailingChar {
        position: usize,
    },
    UnexpectedValueChar {
        kind: JsonValueKind,
        position: usize,
    },

    // TODO: remove?
    UnmatchedArrayClose {
        position: usize,
    },
    UnmatchedObjectClose {
        position: usize,
    },
    InvalidNumber {
        position: usize,
        // TODO: error_position? or range
    },
    InvalidObject {
        position: usize,
        // TODO: error_position? or range
    },
    UnexpectedKind {
        expected_kinds: &'static [JsonValueKind],
        actual_kind: JsonValueKind,
        position: usize, // TODO: range
    },
    // Valid JSON value, but the content was unexpected.
    UnexpectedValue {
        kind: JsonValueKind,
        position: usize,
        error: Box<dyn Send + Sync + std::error::Error>,
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
            JsonParseError::UnexpectedEos { position }
            | JsonParseError::UnexpectedLeadingChar { position }
            | JsonParseError::UnexpectedTrailingChar { position }
            | JsonParseError::UnexpectedValueChar { position, .. }
            | JsonParseError::UnmatchedArrayClose { position }
            | JsonParseError::UnmatchedObjectClose { position }
            | JsonParseError::InvalidNumber { position }
            | JsonParseError::InvalidObject { position }
            | JsonParseError::UnexpectedKind { position, .. }
            | JsonParseError::UnexpectedValue { position, .. }
            | JsonParseError::UnexpectedArraySize { position, .. }
            | JsonParseError::MissingRequiredMember { position, .. } => *position,
        }
    }

    // TODO: row_column_line()
    // TDOO: get_value()
}
