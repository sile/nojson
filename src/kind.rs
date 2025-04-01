#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsonValueKind {
    Null,
    Bool,
    Integer,
    Float,
    String,
    Array,
    Object,
}

impl JsonValueKind {
    pub const fn is_null(self) -> bool {
        matches!(self, Self::Null)
    }

    pub const fn is_bool(self) -> bool {
        matches!(self, Self::Bool)
    }

    pub const fn is_integer(self) -> bool {
        matches!(self, Self::Integer)
    }

    pub const fn is_float(self) -> bool {
        matches!(self, Self::Float)
    }

    pub const fn is_number(self) -> bool {
        matches!(self, Self::Integer | Self::Float)
    }

    pub const fn is_string(self) -> bool {
        matches!(self, Self::String)
    }

    pub const fn is_array(self) -> bool {
        matches!(self, Self::Array)
    }

    pub const fn is_object(self) -> bool {
        matches!(self, Self::Object)
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            JsonValueKind::Null => "null",
            JsonValueKind::Bool => "boolean",
            JsonValueKind::Integer => "number",
            JsonValueKind::Float => "float",
            JsonValueKind::String => "string",
            JsonValueKind::Array => "array",
            JsonValueKind::Object => "object",
        }
    }
}
