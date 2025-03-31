use crate::JsonValueKind;

/// JSON parse error.
#[derive(Debug)]
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
}

impl JsonParseError {
    pub fn position(&self) -> usize {
        match self {
            JsonParseError::UnexpectedEos { position, .. }
            | JsonParseError::UnexpectedTrailingChar { position, .. }
            | JsonParseError::UnexpectedValueChar { position, .. }
            | JsonParseError::InvalidValue { position, .. } => *position,
        }
    }

    // TODO: row_column_line()
    // TDOO: get_value()
}
