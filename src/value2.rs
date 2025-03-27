use std::str::FromStr;

use crate::value::Json;

pub const WHITESPACES: [char; 4] = [' ', '\t', '\r', '\n'];
pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

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
pub enum JsonValue<'a> {
    Null,
    Bool(bool),
    Number(JsonNumber<'a>),
    String(JsonString<'a>),
    Array(JsonArray<'a>),
}

impl<'a> JsonValue<'a> {
    pub fn from_str_borrowed(text: &'a str) -> Result<Self, Error> {
        let text = text.trim_matches(WHITESPACES); // TODO: remove?
        match text {
            "null" => Ok(Self::Null),
            "true" => Ok(Self::Bool(true)),
            "false" => Ok(Self::Bool(false)),
            _ => {
                let c = text.chars().next().expect("TODO");
                match c {
                    '-' | '0' => JsonNumber::from_str_borrowed(text).map(Self::Number),
                    '"' => JsonString::from_str_borrowed(text).map(Self::String),
                    '[' => JsonArray::from_str_borrowed(text).map(Self::Array),
                    '{' => todo!(),
                    _ => todo!(),
                }
            }
        }
    }

    pub fn parse<T>(&self) -> Result<T, T::Err>
    where
        T: Json + FromStr,
        Error: From<T::Err>,
    {
        todo!()
    }

    // parse_nullable

    // TODO: JsonValueOwned?
    pub fn to_owned(&self) -> JsonValue<'static> {
        todo!()
    }
}

impl FromStr for JsonValue<'static> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = JsonValue::from_str_borrowed(s)?;
        Ok(value.to_owned())
    }
}

#[derive(Debug, Clone)]
pub struct JsonString<'a> {
    pub text: &'a str,
    pub unescaped_text: Option<String>,
}

impl<'a> JsonString<'a> {
    pub fn from_str_borrowed(text: &'a str) -> Result<Self, Error> {
        let s = text.strip_prefix('"').expect("TODO");
        let s = s.strip_suffix('"').expect("TODO");
        if !s.contains(['"', '\\']) {
            return Ok(Self {
                text,
                unescaped_text: None,
            });
        }

        let mut unescaped = String::with_capacity(text.len());
        unescaped.push('"');
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                '"' => todo!(),
                '\\' => {
                    let c = chars.next().expect("TODO");
                    match c {
                        '\\' => unescaped.push('\\'),
                        '"' => unescaped.push('"'),
                        'n' => unescaped.push('\n'),
                        'r' => unescaped.push('\r'),
                        't' => unescaped.push('\t'),
                        'b' => unescaped.push('\x08'),
                        'f' => unescaped.push('\x0C'),
                        'u' => {
                            let mut code_point = 0;
                            for _ in 0..4 {
                                let hex_char = chars.next().expect("TODO");
                                let digit = hex_char.to_digit(16).expect("TODO");
                                code_point = (code_point << 4) | digit;
                            }
                            unescaped.push(char::from_u32(code_point).expect("TODO"));
                        }
                        _ => todo!(),
                    }
                }
                _ => unescaped.push(c),
            }
        }
        unescaped.push('"');

        Ok(Self {
            text,
            unescaped_text: Some(unescaped),
        })
    }
}

#[derive(Debug, Clone)]
pub struct JsonNumber<'a> {
    pub text: &'a str,
}

impl<'a> JsonNumber<'a> {
    pub fn from_str_borrowed(text: &'a str) -> Result<Self, Error> {
        let s = text.strip_prefix('-').unwrap_or(text);
        let s = s.strip_prefix(DIGITS).expect("TODO");
        let valid = if let Some((s0, s1)) = s.split_once('.') {
            s1.ends_with(DIGITS) && s0.chars().chain(s1.chars()).all(|c| c.is_ascii_digit())
        } else {
            s.chars().all(|c| c.is_ascii_digit())
        };
        if !valid {
            todo!()
        }
        Ok(Self { text })
    }
}

#[derive(Debug, Clone)]
pub struct JsonArray<'a> {
    pub text: &'a str,
    // TODO: rename
    pub elements: Vec<JsonValue<'a>>,
}

impl<'a> JsonArray<'a> {
    pub fn from_str_borrowed(text: &'a str) -> Result<Self, Error> {
        let s = text.strip_prefix('[').expect("TODO");
        let s = s.strip_suffix(']').expect("TODO");
        let s = s.trim_matches(WHITESPACES);

        let mut elements = Vec::new();
        if s.is_empty() {
            return Ok(Self { text, elements });
        }

        todo!()
    }
}
