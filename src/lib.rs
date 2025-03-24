#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JsonValueKind {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl JsonValueKind {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'n' => Some(Self::Null),
            't' | 'f' => Some(Self::Bool),
            '0'..='9' => Some(Self::Number),
            '"' => Some(Self::String),
            '[' => Some(Self::Array),
            '{' => Some(Self::Object),
            _ => None,
        }
    }
}
