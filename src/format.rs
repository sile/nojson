use std::fmt::{Display, Write};

use crate::DisplayJson;

pub struct JsonArrayFormatter;
pub struct JsonObjectFormatter;

// TODO: Debug
pub struct JsonFormatter<'a, 'b> {
    // TODO: private
    pub inner: &'a mut std::fmt::Formatter<'b>,
    pub indent: usize,
    pub space: usize,
    pub level: usize,
}

impl<'a, 'b> JsonFormatter<'a, 'b> {
    pub fn new(inner: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self {
            inner,
            indent: 0,
            space: 0,
            level: 0,
        }
    }
}

impl<'b> JsonFormatter<'_, 'b> {
    pub fn value<T: DisplayJson>(&mut self, value: T) -> std::fmt::Result {
        value.fmt(self)
    }

    pub fn string<T: Display>(&mut self, content: T) -> std::fmt::Result {
        self.inner.write_char('"')?;
        {
            let mut fmt = JsonStringContentFormatter { inner: self.inner };
            write!(fmt, "{content}")?;
        }
        self.inner.write_char('"')?;
        Ok(())
    }

    pub fn inner_mut(&mut self) -> &mut std::fmt::Formatter<'b> {
        self.inner
    }

    pub fn write_array_start(&mut self) -> std::fmt::Result {
        write!(self.inner, "[")?;
        self.level += 1;
        Ok(())
    }

    pub fn write_array_element<T>(&mut self, _value: T, first: bool) -> std::fmt::Result
    where
        T: DisplayJson,
    {
        if !first {
            write!(self.inner, ",")?;
        }

        if self.indent > 0 {
            let indent = self.indent * self.level;
            write!(self.inner, "\n{:indent$}", "", indent = indent)?;
        }

        // TODO: write value

        Ok(())
    }

    pub fn write_array_end(&mut self, empty: bool) -> std::fmt::Result {
        self.level -= 1;
        if !empty && self.indent > 0 {
            let indent = self.indent * self.level;
            write!(self.inner, "\n{:indent$}", "", indent = indent)?;
        }
        write!(self.inner, "]")?;
        Ok(())
    }
}

struct JsonStringContentFormatter<'a, 'b> {
    inner: &'a mut std::fmt::Formatter<'b>,
}

impl std::fmt::Write for JsonStringContentFormatter<'_, '_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            match c {
                '"' => write!(self.inner, r#"\""#)?,
                '\\' => write!(self.inner, r#"\\"#)?,
                '\n' => write!(self.inner, r#"\n"#)?,
                '\r' => write!(self.inner, r#"\r"#)?,
                '\t' => write!(self.inner, r#"\t"#)?,
                '\u{0008}' => write!(self.inner, r#"\b"#)?,
                '\u{000C}' => write!(self.inner, r#"\f"#)?,
                _ if c.is_ascii_control() => write!(self.inner, "\\u{:04x}", c as u32)?,
                _ => write!(self.inner, "{c}")?,
            }
        }
        Ok(())
    }
}
