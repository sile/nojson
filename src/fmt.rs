use std::fmt::{Display, Write};

// TODO: Debug
pub struct JsonFormatter<'a> {
    // TODO: private
    pub inner: &'a mut std::fmt::Formatter<'a>,
    pub indent: usize,
    pub space: usize,
    pub level: usize,
}

impl<'a> JsonFormatter<'a> {
    pub fn new(fmt: &'a mut std::fmt::Formatter<'a>) -> Self {
        Self {
            inner: fmt,
            indent: 0,
            space: 0,
            level: 0,
        }
    }
}

impl<'a> JsonFormatter<'a> {
    pub fn write_value<T: Display>(&mut self, value: T) -> std::fmt::Result {
        write!(self.inner, "{value}")
    }

    pub fn write_string<T: Display>(&mut self, content: T) -> std::fmt::Result {
        write!(self.inner, "\"")?;
        {
            let mut fmt = JsonStringContentFormatter { inner: self.inner };
            write!(fmt, "{content}")?;
        }
        write!(self.inner, "\"")?;
        Ok(())
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

impl<'a, 'b> std::fmt::Write for JsonStringContentFormatter<'a, 'b> {
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

pub trait DisplayJson {
    fn fmt(&self, f: &mut JsonFormatter<'_>) -> std::fmt::Result;
}

impl<T: DisplayJson> DisplayJson for &T {
    fn fmt(&self, f: &mut JsonFormatter<'_>) -> std::fmt::Result {
        (*self).fmt(f)
    }
}

impl<T: DisplayJson> DisplayJson for Box<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl DisplayJson for bool {
    fn fmt(&self, f: &mut JsonFormatter<'_>) -> std::fmt::Result {
        f.write_value(self)
    }
}

impl DisplayJson for i8 {
    fn fmt(&self, f: &mut JsonFormatter<'_>) -> std::fmt::Result {
        f.write_value(self)
    }
}

// impl DisplayJson for u8 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for i16 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for u16 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for i32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for u32 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for i64 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for u64 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for i128 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for u128 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for isize {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for usize {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self}")
//     }
// }

// impl DisplayJson for &str {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "\"")?;
//         for c in self.chars() {
//             match c {
//                 '\n' => write!(f, r#"\n"#)?,
//                 '\r' => write!(f, r#"\r"#)?,
//                 '\t' => write!(f, r#"\t"#)?,
//                 '\\' => write!(f, r#"\\"#)?,
//                 '\"' => write!(f, r#"\""#)?,
//                 '\x08' => write!(f, r#"\b"#)?,
//                 '\x0C' => write!(f, r#"\f"#)?,
//                 c if c.is_control() => write!(f, r#"\u{:04x}"#, c as u32)?,
//                 _ => write!(f, "{c}")?,
//             }
//         }
//         write!(f, "\"")?;
//         Ok(())
//     }
// }

// impl DisplayJson for String {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.as_str().fmt(f)
//     }
// }

// impl<T: DisplayJson> DisplayJson for &[T] {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         JsonArrayFormatter::new(f).values(self.iter()).finish()
//     }
// }

// impl<T: DisplayJson, const N: usize> DisplayJson for [T; N] {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         JsonArrayFormatter::new(f).values(self.iter()).finish()
//     }
// }

// impl<T: DisplayJson> DisplayJson for Vec<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         JsonArrayFormatter::new(f).values(self.iter()).finish()
//     }
// }

// impl<T: DisplayJson> DisplayJson for VecDeque<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         JsonArrayFormatter::new(f).values(self.iter()).finish()
//     }
// }

// impl<K: Display, V: DisplayJson> DisplayJson for BTreeMap<K, V> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         JsonObjectFormatter::new(f).members(self.iter()).finish()
//     }
// }

// impl<K: Display, V: DisplayJson> DisplayJson for HashMap<K, V> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         JsonObjectFormatter::new(f).members(self.iter()).finish()
//     }
// }

// impl<T: DisplayJson> DisplayJson for Option<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Some(v) = self {
//             write!(f, "{}", Json(v))
//         } else {
//             write!(f, "null")
//         }
//     }
// }
