use std::{borrow::Cow, str::FromStr};

use crate::value::Json;

#[derive(Debug)]
pub struct Error {
    pub path: Vec<PathItem>,
    pub reason: ErrorReason,
    pub cause: Option<Box<dyn 'static + std::error::Error>>,
}

#[derive(Debug)]
pub enum ErrorReason {}

#[derive(Debug)]
pub enum PathItem {
    Index(usize),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Number(JsonNumber<'a>),
    String(JsonString<'a>),
}

impl<'a> Value<'a> {
    pub fn parse<T>(&self) -> Result<T, T::Err>
    where
        T: Json + FromStr,
        Error: From<T::Err>,
    {
        todo!()
    }

    // parse_nullable
}

#[derive(Debug, Clone)]
pub struct JsonString<'a> {
    pub text: Cow<'a, str>,
}

#[derive(Debug, Clone)]
pub struct JsonNumber<'a> {
    pub text: Cow<'a, str>,
}
