use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

use value::{JsonF64, JsonString};

pub mod value;

#[derive(Debug)]
pub enum Error {
    Eos,
    InvalidJsonValue,
    NotNull,
    NotBool,
    NotNumber,
    NotFiniteFloat, // TODO
    NotValidFloat,
    NotString, // TODO
    NotValidString,
    NotValidArray,
}

impl From<ParseBoolError> for Error {
    fn from(_value: ParseBoolError) -> Self {
        Self::NotBool
    }
}

impl From<ParseIntError> for Error {
    fn from(_value: ParseIntError) -> Self {
        Self::NotNumber
    }
}

impl From<ParseFloatError> for Error {
    fn from(_value: ParseFloatError) -> Self {
        Self::NotNumber
    }
}

pub fn float<T>(value: T) -> Option<JsonF64>
where
    f64: From<T>,
{
    JsonF64::new(value.into())
}

pub fn string<T>(value: T) -> JsonString<T> {
    JsonString::new(value)
}
