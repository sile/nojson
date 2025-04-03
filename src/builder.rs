use std::fmt::Display;

use crate::DisplayJson;

pub struct JsonFormatter<'a> {
    fmt: &'a mut std::fmt::Formatter<'a>,
}

impl<'a> JsonFormatter<'a> {
    pub fn new(fmt: &'a mut std::fmt::Formatter<'a>) -> Self {
        Self { fmt }
    }

    pub fn null(self) -> std::fmt::Result {
        write!(self.fmt, "null")
    }

    pub fn bool(self, v: bool) -> std::fmt::Result {
        write!(self.fmt, "{v}")
    }

    pub fn integer<T>(self, v: T) -> std::fmt::Result
    where
        // TODO: TryFrom
        i64: From<T>,
    {
        write!(self.fmt, "{}", i64::from(v))
    }

    pub fn float<T>(self, v: T) -> std::fmt::Result
    where
        f64: From<T>,
    {
        // TODO: check finite
        write!(self.fmt, "{}", f64::from(v))
    }

    pub fn string<T>(self, _v: T) -> std::fmt::Result
    where
        T: Display,
    {
        todo!()
    }

    pub fn value<T>(self, _v: T) -> std::fmt::Result
    where
        T: DisplayJson,
    {
        todo!()
    }
}
