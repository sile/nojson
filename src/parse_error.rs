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
