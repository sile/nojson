use crate::JsonValueKind;

/// JSON parse error.
#[derive(Debug)]
#[non_exhaustive]
pub enum JsonParseError {
    UnexpectedEos {
        position: usize,
    },
    UnexpectedTrailingChar {
        position: usize,
    },
    UnexpectedValueChar {
        kind: Option<JsonValueKind>,
        position: usize,
    },

    UnexpectedKind {
        expected_kinds: &'static [JsonValueKind],
        actual_kind: JsonValueKind,
        position: usize, // TODO: range
    },
    // Valid JSON value, but the content was unexpected.
    // TODO rename
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
            | JsonParseError::UnexpectedTrailingChar { position }
            | JsonParseError::UnexpectedValueChar { position, .. }
            | JsonParseError::UnexpectedKind { position, .. }
            | JsonParseError::UnexpectedValue { position, .. }
            | JsonParseError::UnexpectedArraySize { position, .. }
            | JsonParseError::MissingRequiredMember { position, .. } => *position,
        }
    }

    // TODO: row_column_line()
    // TDOO: get_value()
}
