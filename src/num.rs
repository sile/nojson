//! Number types for JSON.
use std::{fmt::Display, hash::Hash};

use crate::{
    fmt::DisplayJson,
    str::{JsonParseError, RawJsonValue},
};

/// A variant of [`f64`] representing finite floating-point values.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Makes a new [`FiniteF64`] instance.
    ///
    /// If the given value is NaN or infinite, this function returns [`None`].
    pub const fn new(v: f64) -> Option<Self> {
        if v.is_finite() {
            // Normalize negative zero for hashing purposes.
            Some(Self(if v == -0.0 { 0.0 } else { v }))
        } else {
            None
        }
    }

    /// Returns the value of this number.
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl Eq for FiniteF64 {}

impl PartialOrd for FiniteF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiniteF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Hash for FiniteF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl DisplayJson for FiniteF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for FiniteF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplayJson::fmt(self, f)
    }
}

// TODO: Add FromJsonValueStr
impl TryFrom<RawJsonValue<'_>> for FiniteF64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'_>) -> Result<Self, Self::Error> {
        value.as_number()?.parse().map(Self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Json;

    use super::*;

    #[test]
    fn finite_f64() {
        let v: Json<FiniteF64> = "3.14".parse().expect("ok");
        assert_eq!(v.0.get(), 3.14);
        assert_eq!(v.to_string(), "3.14");
    }
}
