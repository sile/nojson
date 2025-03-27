use std::{borrow::Cow, str::FromStr};

pub const WHITESPACES: [char; 4] = [' ', '\t', '\r', '\n'];
pub const NUMBER_PREFIX: [char; 11] = ['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
pub struct Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Null,
    Bool,
    Integer,
    Float,
    String,
    StringEscaped,
    Array,
    Object,
}

pub trait ParseJson: Sized {
    fn parse_json(text: &str) -> Result<Self, Error>;
}

impl<'a, 'b, T: TryFrom<JsonText<'a, 'b>>> ParseJson for T {
    fn parse_json(_text: &str) -> Result<Self, Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Json<T>(pub T);

impl<'a, 'b, T: TryFrom<JsonText<'a, 'b>>> FromStr for Json<T> {
    type Err = Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

// TODO: rename
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsonValue {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
    pub scope: usize, // TODO: rename
}

#[derive(Debug)]
pub struct JsonParser<'a> {
    pub text: &'a str,
    pub index: usize,
    pub values: Vec<JsonValue>,
}

impl<'a> JsonParser<'a> {
    pub fn parse(&mut self) -> Result<(), Error> {
        self.strip_whitespaces();

        if self.text.starts_with("null") {
            self.push_value(Kind::Null, "null".len());
        } else if self.text.starts_with("true") {
            self.push_value(Kind::Bool, "true".len());
        } else if self.text.starts_with("false") {
            self.push_value(Kind::Bool, "false".len());
        } else if self.text.starts_with(NUMBER_PREFIX) {
            self.parse_number()?;
        } else if let Some(s) = self.text.strip_prefix('"') {
            self.parse_string(s)?;
        } else if let Some(s) = self.text.strip_prefix('[') {
            self.parse_array(s)?;
        } else if let Some(s) = self.text.strip_prefix('{') {
            self.parse_object(s)?;
        }
        Ok(())
    }

    fn parse_object(&mut self, mut s: &'a str) -> Result<(), Error> {
        let i = self.values.len();
        self.push_value(Kind::Object, 0);

        loop {
            s = s.trim_start_matches(WHITESPACES);
            if let Some(s) = s.strip_prefix('}') {
                self.proceed(s);
                self.values[i].end = self.index;
                self.values[i].scope = self.values.len() - i;
                return Ok(());
            }

            self.proceed(s);
            s = s.strip_prefix('"').expect("TODO");
            self.parse_string(s)?;
            s = self.text;

            s = s.trim_start_matches(WHITESPACES);
            s = s.strip_prefix(':').expect("TODO");
            s = s.trim_start_matches(WHITESPACES);

            self.proceed(s);
            self.parse()?;
            s = self.text;

            s = s.trim_start_matches(WHITESPACES);
            if s.starts_with('}') {
                continue;
            }
            s = s.strip_prefix(',').expect("TODO");
        }
    }

    fn parse_array(&mut self, mut s: &'a str) -> Result<(), Error> {
        let i = self.values.len();
        self.push_value(Kind::Array, 0);

        loop {
            s = s.trim_start_matches(WHITESPACES);
            if let Some(s) = s.strip_prefix(']') {
                self.proceed(s);
                self.values[i].end = self.index;
                self.values[i].scope = self.values.len() - i;
                return Ok(());
            }
            self.proceed(s);
            self.parse()?;
            s = self.text;

            s = s.trim_start_matches(WHITESPACES);
            if s.starts_with(']') {
                continue;
            }
            s = s.strip_prefix(',').expect("TODO");
        }
    }

    fn proceed(&mut self, s: &'a str) {
        self.index += self.text.len() - s.len();
        self.text = s;
    }

    fn parse_string(&mut self, s: &str) -> Result<(), Error> {
        let mut kind = Kind::String;
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    let n = self.text.len() - s.len();
                    self.push_value(kind, n);
                    return Ok(());
                }
                '\\' => {
                    kind = Kind::StringEscaped;
                    let c = chars.next().expect("TODO");
                    match c {
                        '\\' | '"' | 'n' | 'r' | 't' | 'b' | 'f' => {}
                        'u' => {
                            let mut code_point = 0;
                            for _ in 0..4 {
                                let hex_char = chars.next().expect("TODO");
                                let digit = hex_char.to_digit(16).expect("TODO");
                                code_point = (code_point << 4) | digit;
                            }
                            char::from_u32(code_point).expect("TODO");
                        }
                        _ => todo!(),
                    }
                }
                _ => {}
            }
        }

        todo!()
    }

    fn parse_number(&mut self) -> Result<(), Error> {
        let s = self.text.strip_prefix('-').unwrap_or(self.text);
        let s = s.strip_prefix(DIGITS).expect("TODO");
        let s = s.trim_start_matches(DIGITS);

        let (kind, s) = if let Some(s) = s.strip_prefix('.') {
            let s = s.strip_prefix(DIGITS).expect("TODO");
            let s = s.trim_start_matches(DIGITS);
            (Kind::Float, s)
        } else {
            (Kind::Integer, s)
        };

        let n = self.text.len() - s.len();
        self.push_value(kind, n);

        Ok(())
    }

    fn push_value(&mut self, kind: Kind, len: usize) {
        let start = self.index;
        let end = start + len;
        self.values.push(JsonValue {
            kind,
            start,
            end,
            scope: 1,
        });
        self.index = end;
        self.text = &self.text[len..];
    }

    fn strip_whitespaces(&mut self) {
        let s = self.text.trim_start_matches(WHITESPACES);
        self.index += self.text.len() - s.len();
        self.text = s;
    }
}

#[derive(Debug)]
pub struct JsonText<'a, 'b> {
    pub text: &'a str,
    pub values: Cow<'b, [JsonValue]>,
}

impl<'a> JsonText<'a, 'static> {
    pub fn new(text: &'a str) -> Result<Self, Error> {
        let mut parser = JsonParser {
            text,
            index: 0,
            values: Vec::new(),
        };
        parser.parse()?;
        Ok(Self {
            text,
            values: Cow::Owned(parser.values),
        })
    }

    fn root(&self) -> &JsonValue {
        &self.values[0]
    }

    pub fn remaining_text(&self) -> &'a str {
        &self.text[self.root().end..]
    }

    pub fn kind(&self) -> Kind {
        self.root().kind
    }

    pub fn nullable<F, T>(&self, f: F) -> Result<Option<T>, Error>
    where
        F: FnOnce(&Self) -> Result<T, Error>,
    {
        (self.kind() != Kind::Null).then(|| f(self)).transpose()
    }

    pub fn expect_bool(&self) -> Result<bool, Error> {
        todo!()
    }

    pub fn parse_integer<T>(&self) -> Result<T, Error>
    where
        T: FromStr,
        Error: From<T::Err>,
    {
        self.parse_integer_with(|text| text.parse())
    }

    pub fn parse_integer_with<F, T, E>(&self, _f: F) -> Result<T, Error>
    where
        F: FnOnce(&str) -> Result<T, E>,
        Error: From<E>,
    {
        todo!()
    }

    pub fn parse_number<T>(&self) -> Result<T, Error>
    where
        T: FromStr,
        Error: From<T::Err>,
    {
        todo!()
    }

    pub fn parse_string<T>(&self) -> Result<T, Error>
    where
        T: FromStr,
        Error: From<T::Err>,
    {
        todo!()
    }

    pub fn expect_array(&self) -> Result<JsonArray, Error> {
        if self.kind() != Kind::Array {
            todo!()
        }
        Ok(JsonArray {
            text: &self.text,
            values: &self.values[1..],
        })
    }

    pub fn expect_object(&self) -> Result<JsonObject, Error> {
        if self.kind() != Kind::Object {
            todo!()
        }
        Ok(JsonObject {
            text: &self.text,
            values: &self.values[1..],
        })
    }
}

#[derive(Debug)]
pub struct JsonArray<'a, 'b> {
    pub text: &'a str,
    pub values: &'b [JsonValue],
}

impl<'a, 'b> JsonArray<'a, 'b> {
    pub fn expect_n<const N: usize>(&self) -> Result<[JsonText<'a, 'b>; N], Error> {
        todo!()
    }
}

impl<'a, 'b> Iterator for JsonArray<'a, 'b> {
    type Item = JsonText<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }

        let v = &self.values[0];
        let values = &self.values[..v.scope];
        self.values = &self.values[v.scope..];
        Some(JsonText {
            text: self.text,
            values: Cow::Borrowed(values),
        })
    }
}

#[derive(Debug)]
pub struct JsonObject<'a, 'b> {
    pub text: &'a str,
    pub values: &'b [JsonValue],
}

impl<'a, 'b> JsonObject<'a, 'b> {
    pub fn with<const N: usize, const M: usize>(
        &self,
        _keys: [&str; N],
        _optional_keys: [&str; M],
    ) -> Result<([JsonText<'a, 'b>; N], [Option<JsonText<'a, 'b>>; N]), Error> {
        todo!()
    }
}

impl<'a, 'b> Iterator for JsonObject<'a, 'b> {
    type Item = (JsonString<'a>, JsonText<'a, 'b>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug)]
pub struct JsonString<'a> {
    pub text: &'a str,
    pub value: JsonValue,
}
