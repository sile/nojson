use std::fmt::{Display, Write};

use crate::DisplayJson;

pub struct JsonFormatter<'a, 'b> {
    inner: &'a mut std::fmt::Formatter<'b>,
    level: usize,
    indent_size: usize,
    spacing: bool,
}

impl<'a, 'b> JsonFormatter<'a, 'b> {
    pub fn new(inner: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self {
            inner,
            level: 0,
            indent_size: 0,
            spacing: false,
        }
    }
}

impl<'a, 'b> JsonFormatter<'a, 'b> {
    pub fn value<T: DisplayJson>(&mut self, value: T) -> std::fmt::Result {
        value.fmt(self)
    }

    pub fn string<T: Display>(&mut self, content: T) -> std::fmt::Result {
        write!(self.inner, "\"")?;
        {
            let mut fmt = JsonStringContentFormatter { inner: self.inner };
            write!(fmt, "{content}")?;
        }
        write!(self.inner, "\"")?;
        Ok(())
    }

    pub fn array<F>(&mut self, f: F) -> std::fmt::Result
    where
        F: FnOnce(&mut JsonArrayFormatter<'a, 'b, '_>) -> std::fmt::Result,
    {
        write!(self.inner, "[")?;

        let indent_size = self.indent_size;
        let spacing = self.spacing;
        self.level += 1;
        let mut array = JsonArrayFormatter {
            fmt: self,
            empty: true,
        };
        f(&mut array)?;
        let empty = array.empty;
        self.level -= 1;
        self.indent_size = indent_size;
        self.spacing = spacing;

        if !empty {
            self.indent()?;
        }
        write!(self.inner, "]")?;

        Ok(())
    }

    pub fn object<F>(&mut self, f: F) -> std::fmt::Result
    where
        F: FnOnce(&mut JsonObjectFormatter<'a, 'b, '_>) -> std::fmt::Result,
    {
        write!(self.inner, "{{")?;

        let indent_size = self.indent_size;
        let spacing = self.spacing;
        self.level += 1;
        let mut object = JsonObjectFormatter {
            fmt: self,
            empty: true,
        };
        f(&mut object)?;
        let empty = object.empty;
        self.level -= 1;
        self.indent_size = indent_size;
        self.spacing = spacing;

        if !empty {
            if self.indent_size > 0 {
                self.indent()?;
            } else if self.spacing {
                write!(self.inner, " ")?;
            }
        }
        write!(self.inner, "}}")?;

        Ok(())
    }

    pub fn inner_mut(&mut self) -> &mut std::fmt::Formatter<'b> {
        self.inner
    }

    fn indent(&mut self) -> std::fmt::Result {
        if self.indent_size > 0 {
            let total = self.indent_size * self.level;
            write!(self.inner, "\n{:total$}", "", total = total)?;
        }
        Ok(())
    }

    pub fn get_level(&self) -> usize {
        self.level
    }

    /// Returns the number of spaces used for each indentation level.
    pub fn get_indent_size(&self) -> usize {
        self.indent_size
    }

    pub fn set_indent_size(&mut self, size: usize) {
        self.indent_size = size;
    }

    /// Returnes whether inserting a space after ':' and ','.
    pub fn get_spacing(&self) -> bool {
        self.spacing
    }

    pub fn set_spacing(&mut self, enable: bool) {
        self.spacing = enable;
    }
}

impl std::fmt::Debug for JsonFormatter<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonFormatter")
            .field("level", &self.level)
            .field("indent_size", &self.indent_size)
            .field("spacing", &self.spacing)
            .finish_non_exhaustive()
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

pub struct JsonArrayFormatter<'a, 'b, 'c> {
    fmt: &'c mut JsonFormatter<'a, 'b>,
    empty: bool,
}

impl JsonArrayFormatter<'_, '_, '_> {
    pub fn element<T: DisplayJson>(&mut self, element: T) -> std::fmt::Result {
        if !self.empty {
            write!(self.fmt.inner, ",")?;
            if self.fmt.spacing && self.fmt.indent_size == 0 {
                write!(self.fmt.inner, " ")?;
            }
        }
        self.fmt.indent()?;
        self.fmt.value(element)?;
        self.empty = false;
        Ok(())
    }

    pub fn elements<I>(&mut self, elements: I) -> std::fmt::Result
    where
        I: IntoIterator,
        I::Item: DisplayJson,
    {
        for element in elements {
            self.element(element)?;
        }
        Ok(())
    }
}

pub struct JsonObjectFormatter<'a, 'b, 'c> {
    fmt: &'c mut JsonFormatter<'a, 'b>,
    empty: bool,
}

impl JsonObjectFormatter<'_, '_, '_> {
    pub fn member<N, V>(&mut self, name: N, value: V) -> std::fmt::Result
    where
        N: Display,
        V: DisplayJson,
    {
        if !self.empty {
            write!(self.fmt.inner, ",")?;
            if self.fmt.spacing && self.fmt.indent_size == 0 {
                write!(self.fmt.inner, " ")?;
            }
        } else if self.fmt.spacing && self.fmt.indent_size == 0 {
            write!(self.fmt.inner, " ")?;
        }

        self.fmt.indent()?;
        self.fmt.string(name)?;
        write!(self.fmt.inner, ":")?;
        if self.fmt.spacing {
            write!(self.fmt.inner, " ")?;
        }
        self.fmt.value(value)?;
        self.empty = false;
        Ok(())
    }

    pub fn members<I, N, V>(&mut self, members: I) -> std::fmt::Result
    where
        I: IntoIterator<Item = (N, V)>,
        N: Display,
        V: DisplayJson,
    {
        for (name, value) in members {
            self.member(name, value)?;
        }
        Ok(())
    }
}
