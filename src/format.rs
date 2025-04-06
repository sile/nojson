use std::fmt::{Display, Write};

use crate::DisplayJson;

/// A formatter for JSON values that controls the layout and formatting of the output.
///
/// `JsonFormatter` wraps a standard `std::fmt::Formatter` and provides methods specifically designed
/// for generating well-formed JSON with customizable formatting options like indentation and spacing.
///
/// This formatter is primarily used when implementing the `DisplayJson` trait or when using the
/// [`json()`](crate::json) function for in-place JSON generation.
///
/// # Examples
///
/// Basic usage with the `json()` function:
/// ```
/// use nojson::json;
///
/// // Generate compact JSON
/// let compact = json(|f| f.value([1, 2, 3]));
/// assert_eq!(compact.to_string(), "[1,2,3]");
///
/// // Generate pretty-printed JSON
/// let pretty = json(|f| {
///     f.set_indent_size(2);
///     f.set_spacing(true);
///     f.value([1, 2, 3])
/// });
///
/// assert_eq!(
///     format!("\n{}", pretty),
///     r#"
/// [
///   1,
///   2,
///   3
/// ]"#
/// );
/// ```
pub struct JsonFormatter<'a, 'b> {
    inner: &'a mut std::fmt::Formatter<'b>,
    level: usize,
    indent_size: usize,
    spacing: bool,
}

impl<'a, 'b> JsonFormatter<'a, 'b> {
    pub(crate) fn new(inner: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self {
            inner,
            level: 0,
            indent_size: 0,
            spacing: false,
        }
    }

    /// Formats a value that implements the `DisplayJson` trait.
    ///
    /// This is the primary method for writing a value to the JSON output.
    ///
    /// # Examples
    ///
    /// ```
    /// use nojson::json;
    ///
    /// let output = json(|f| f.value([1, 2, 3]));
    /// assert_eq!(output.to_string(), "[1,2,3]");
    /// ```
    pub fn value<T: DisplayJson>(&mut self, value: T) -> std::fmt::Result {
        value.fmt(self)
    }

    /// Formats a value as a JSON string with proper escaping.
    ///
    /// This method handles the necessary escaping of special characters in JSON strings,
    /// including quotes, backslashes, control characters, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use nojson::json;
    ///
    /// let output = json(|f| f.string("Hello\nWorld"));
    /// assert_eq!(output.to_string(), r#""Hello\nWorld""#);
    /// ```
    pub fn string<T: Display>(&mut self, content: T) -> std::fmt::Result {
        write!(self.inner, "\"")?;
        {
            let mut fmt = JsonStringContentFormatter { inner: self.inner };
            write!(fmt, "{content}")?;
        }
        write!(self.inner, "\"")?;
        Ok(())
    }

    /// Creates a JSON array with the provided formatting function.
    ///
    /// This method starts a new JSON array and provides a `JsonArrayFormatter` to the callback
    /// function for adding elements to the array. It handles proper indentation, spacing, and
    /// brackets placement.
    ///
    /// # Examples
    ///
    /// ```
    /// use nojson::json;
    ///
    /// let output = json(|f| {
    ///     f.array(|f| {
    ///         f.element(1)?;
    ///         f.element(2)?;
    ///         f.element(3)
    ///     })
    /// });
    /// assert_eq!(output.to_string(), "[1,2,3]");
    ///
    /// // With pretty-printing
    /// let pretty = json(|f| {
    ///     f.set_indent_size(2);
    ///     f.set_spacing(true);
    ///     f.array(|f| {
    ///         f.element(1)?;
    ///         f.element(2)?;
    ///         f.element(3)
    ///     })
    /// });
    /// ```
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

    /// Creates a JSON object with the provided formatting function.
    ///
    /// This method starts a new JSON object and provides a `JsonObjectFormatter` to the callback
    /// function for adding members to the object. It handles proper indentation, spacing, and
    /// braces placement.
    ///
    /// # Examples
    ///
    /// ```
    /// use nojson::json;
    /// use std::collections::HashMap;
    ///
    /// let output = json(|f| {
    ///     f.object(|f| {
    ///         f.member("name", "Alice")?;
    ///         f.member("age", 30)
    ///     })
    /// });
    /// assert_eq!(output.to_string(), r#"{"name":"Alice","age":30}"#);
    ///
    /// // With pretty-printing
    /// let pretty = json(|f| {
    ///     f.set_indent_size(2);
    ///     f.set_spacing(true);
    ///     f.object(|f| {
    ///         f.member("name", "Alice")?;
    ///         f.member("age", 30)
    ///     })
    /// });
    /// ```
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

    /// Returns a mutable reference to the inner `std::fmt::Formatter`.
    ///
    /// This method provides direct access to the wrapped standard formatter, which can be useful
    /// for implementing custom formatting logic for primitive types.
    pub fn inner_mut(&mut self) -> &mut std::fmt::Formatter<'b> {
        self.inner
    }

    /// Returns the current indentation level.
    ///
    /// The indentation level increases when entering arrays and objects, and
    /// decreases when exiting them.
    pub fn get_level(&self) -> usize {
        self.level
    }

    /// Returns the number of spaces used for each indentation level.
    pub fn get_indent_size(&self) -> usize {
        self.indent_size
    }

    /// Sets the number of spaces used for each indentation level.
    ///
    /// Note that this setting only affects the current and higher indentation levels.
    pub fn set_indent_size(&mut self, size: usize) {
        self.indent_size = size;
    }

    /// Returnes whether inserting a space after ':', ',', and '{'.
    pub fn get_spacing(&self) -> bool {
        self.spacing
    }

    /// Sets whether inserting a space after ':', ',', and '{'.
    ///
    /// Note that this setting only affects the current and higher indentation levels.
    pub fn set_spacing(&mut self, enable: bool) {
        self.spacing = enable;
    }

    fn indent(&mut self) -> std::fmt::Result {
        if self.indent_size > 0 {
            let total = self.indent_size * self.level;
            write!(self.inner, "\n{:total$}", "", total = total)?;
        }
        Ok(())
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
