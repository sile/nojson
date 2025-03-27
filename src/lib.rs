use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

pub mod formatter;
//pub mod value2;
pub mod value3;

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
