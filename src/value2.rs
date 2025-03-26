use std::str::FromStr;

#[derive(Debug)]
pub enum Value<'a> {
    Null(Null<'a>),
    Bool(Bool<'a>),
}

impl<'a> Value<'a> {
    pub fn parse<T: FromStr>(&self) -> Result<T, T::Err> {
        match self {
            Value::Null(v) => v.text.parse(),
            Value::Bool(v) => v.text.parse(),
        }
    }
}

#[derive(Debug)]
pub struct Null<'a> {
    pub text: &'a str,
}

impl Null<'static> {
    pub const fn new() -> Self {
        Self { text: "null" }
    }
}

#[derive(Debug)]
pub struct Bool<'a> {
    pub value: bool,
    pub text: &'a str,
}
