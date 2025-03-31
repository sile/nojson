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

    // TODO: row_column_line()
    // TDOO: get_value()
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
