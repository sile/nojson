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

    pub fn null(self) -> std::fmt::Result {
        todo!()
    }

    // TODO: array<F>(&mut self, f:F)-> std::fmt::Result {}
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

    pub fn value_with<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut JsonFormatter<W>) -> std::fmt::Result,
    {
        if self.error.is_some() {
            return self;
        }
        if self.first {
            self.first = false;
            self.error = self.fmt.writer.write_char(',').err();
            if self.error.is_some() {
                return self;
            }
        }
        self.error = f(self.fmt).err();
        self
    }

    pub fn value<T: JsonDisplay>(&mut self, value: &T) -> &mut Self {
        self.value_with(|fmt| write!(fmt.writer, "{value}"))
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
