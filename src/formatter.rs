use std::fmt::{Display, Write};

pub trait JsonDisplay: Display {}

pub trait JsonStringDisplay: JsonDisplay {}

impl JsonDisplay for bool {}

impl JsonDisplay for usize {}

#[derive(Debug)]
pub struct JsonF64(pub f64);

#[derive(Debug)]
pub struct JsonStr<T: AsRef<str>>(pub T);

#[derive(Debug)]
pub struct JsonFormatter<W> {
    writer: W,
    // TODO: indent, space
}

impl<W: Write> JsonFormatter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn array(&mut self) -> JsonArrayFormatter<W> {
        JsonArrayFormatter::new(self)
    }
}

#[derive(Debug)]
pub struct JsonArrayFormatter<'a, W> {
    fmt: &'a mut JsonFormatter<W>,
    error: Option<std::fmt::Error>,
    first: bool,
}

impl<'a, W: Write> JsonArrayFormatter<'a, W> {
    fn new(fmt: &'a mut JsonFormatter<W>) -> Self {
        let error = fmt.writer.write_char('[').err();
        Self {
            fmt,
            error,
            first: true,
        }
    }

    pub fn value<T: JsonDisplay>(&mut self, value: &T) -> &mut Self {
        if self.error.is_some() {
        } else if self.first {
            self.first = false;
            self.error = write!(self.fmt.writer, "{value}").err();
        } else {
            self.error = write!(self.fmt.writer, ",{value}").err();
        }
        self
    }

    pub fn values<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: JsonDisplay,
    {
        for value in values {
            self.value(&value);
        }
        self
    }

    pub fn finish(self) -> std::fmt::Result {
        if let Some(e) = self.error {
            Err(e)
        } else {
            self.fmt.writer.write_char(']')
        }
    }
}
