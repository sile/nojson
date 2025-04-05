/// Enum representing the possible types of JSON values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsonValueKind {
    /// `null`.
    Null,

    /// `true` or `false`.
    Boolean,

    /// Integer number.
    Integer,

    /// Floating-point number.
    Float,

    /// String.
    String,

    /// Array.
    Array,

    /// Object.
    Object,
}

impl JsonValueKind {
    /// Returns `true` if this kind is [`JsonValueKind::Null`].
    pub const fn is_null(self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if this kind is [`JsonValueKind::Boolean`].
    pub const fn is_bool(self) -> bool {
        matches!(self, Self::Boolean)
    }

    /// Returns `true` if this kind is [`JsonValueKind::Integer`].
    pub const fn is_integer(self) -> bool {
        matches!(self, Self::Integer)
    }

    /// Returns `true` if this kind is [`JsonValueKind::Float`].
    pub const fn is_float(self) -> bool {
        matches!(self, Self::Float)
    }

    /// Returns `true` if this kind is either [`JsonValueKind::Integer`] or [`JsonValueKind::Float`].
    pub const fn is_number(self) -> bool {
        matches!(self, Self::Integer | Self::Float)
    }

    /// Returns `true` if this kind is [`JsonValueKind::String`].
    pub const fn is_string(self) -> bool {
        matches!(self, Self::String)
    }

    /// Returns `true` if this kind is [`JsonValueKind::Array`].
    pub const fn is_array(self) -> bool {
        matches!(self, Self::Array)
    }

    /// Returns `true` if this kind is [`JsonValueKind::Object`].
    pub const fn is_object(self) -> bool {
        matches!(self, Self::Object)
    }
}
