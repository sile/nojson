use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt::Display,
};

use crate::Json;

#[derive(Debug, Default)]
pub struct JsonFormatter {
    pub indent: usize,
    pub space: usize,
}

impl JsonFormatter {
    // TODO:
    // pub fn array<I>(&mut self, _values: I) -> std::fmt::Result
    // where
    //     I: IntoIterator,
    //     I::Item: DisplayJson,
    // {
    //     todo!()
    // }

    // pub fn object<I, K, V>(&mut self, _members: I) -> std::fmt::Result
    // where
    //     I: IntoIterator<Item = (K, V)>,
    //     K: Display,
    //     V: DisplayJson,
    // {
    //     todo!()
    // }

    // pub fn array_with<F>(&mut self, _f: F) -> std::fmt::Result
    // where
    //     F: Fn(&dyn DisplayJson) -> std::fmt::Result,
    // {
    //     todo!()
    // }

    // pub fn object_with<F, K, V>(&mut self, _f: F) -> std::fmt::Result
    // where
    //     F: Fn(K, V) -> std::fmt::Result,
    //     K: Display,
    //     V: DisplayJson,
    // {
    //     todo!()
    // }
}

// TODO: Formatter and PrettyJson for pretty-print

pub trait DisplayJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<T: DisplayJson> DisplayJson for &T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*self).fmt(f)
    }
}

impl<T: DisplayJson> DisplayJson for Box<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl DisplayJson for bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for i8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for u8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for i16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for u16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for i32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for u32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for i64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for u64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for i128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for u128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for isize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl DisplayJson for &str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for c in self.chars() {
            match c {
                '\n' => write!(f, r#"\n"#)?,
                '\r' => write!(f, r#"\r"#)?,
                '\t' => write!(f, r#"\t"#)?,
                '\\' => write!(f, r#"\\"#)?,
                '\"' => write!(f, r#"\""#)?,
                '\x08' => write!(f, r#"\b"#)?,
                '\x0C' => write!(f, r#"\f"#)?,
                c if c.is_control() => write!(f, r#"\u{:04x}"#, c as u32)?,
                _ => write!(f, "{c}")?,
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}

impl DisplayJson for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<T: DisplayJson> DisplayJson for &[T] {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        JsonArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<T: DisplayJson, const N: usize> DisplayJson for [T; N] {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        JsonArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<T: DisplayJson> DisplayJson for Vec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        JsonArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<T: DisplayJson> DisplayJson for VecDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        JsonArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<K: Display, V: DisplayJson> DisplayJson for BTreeMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        JsonObjectFormatter::new(f).members(self.iter()).finish()
    }
}

impl<K: Display, V: DisplayJson> DisplayJson for HashMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        JsonObjectFormatter::new(f).members(self.iter()).finish()
    }
}

impl<T: DisplayJson> DisplayJson for Option<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = self {
            write!(f, "{}", Json(v))
        } else {
            write!(f, "null")
        }
    }
}

pub struct JsonArrayFormatter<'a, 'b> {
    inner: &'a mut std::fmt::Formatter<'b>,
    first: bool,
    error: Option<std::fmt::Error>,
}

impl<'a, 'b> JsonArrayFormatter<'a, 'b> {
    pub fn new(inner: &'a mut std::fmt::Formatter<'b>) -> Self {
        let error = write!(inner, "[").err();
        Self {
            inner,
            first: true,
            error,
        }
    }

    pub fn value_with<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut std::fmt::Formatter<'b>) -> std::fmt::Result,
    {
        if self.error.is_some() {
            return self;
        }

        if !self.first {
            self.error = write!(self.inner, ",").err();
            if self.error.is_some() {
                return self;
            }
        } else {
            self.first = false;
        }

        self.error = f(self.inner).err();
        if self.error.is_some() {
            return self;
        }

        self
    }

    pub fn value<T>(&mut self, value: T) -> &mut Self
    where
        T: DisplayJson,
    {
        self.value_with(|f| write!(f, "{}", Json(value)))
    }

    pub fn values<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: DisplayJson,
    {
        if self.error.is_some() {
            return self;
        }
        for v in iter {
            if self.value(v).error.is_some() {
                break;
            }
        }
        self
    }

    pub fn finish(&mut self) -> std::fmt::Result {
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        write!(self.inner, "]")?;
        Ok(())
    }
}

pub struct JsonObjectFormatter<'a, 'b> {
    inner: &'a mut std::fmt::Formatter<'b>,
    first: bool,
    error: Option<std::fmt::Error>,
}

impl<'a, 'b> JsonObjectFormatter<'a, 'b> {
    pub fn new(inner: &'a mut std::fmt::Formatter<'b>) -> Self {
        let error = write!(inner, "{{").err();
        Self {
            inner,
            first: true,
            error,
        }
    }

    pub fn member_with<K, F>(&mut self, key: K, f: F) -> &mut Self
    where
        K: Display,
        F: FnOnce(&mut std::fmt::Formatter<'b>) -> std::fmt::Result,
    {
        if self.error.is_some() {
            return self;
        }

        if !self.first {
            self.error = write!(self.inner, ",").err();
            if self.error.is_some() {
                return self;
            }
        } else {
            self.first = false;
        }

        // TODO: escape `key` if need
        self.error = write!(self.inner, "\"{}\":", key)
            .and_then(|()| f(self.inner))
            .err();
        if self.error.is_some() {
            return self;
        }

        self
    }

    pub fn member<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Display,
        V: DisplayJson,
    {
        self.member_with(key, |f| write!(f, "{}", Json(value)))
    }

    pub fn members<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Display,
        V: DisplayJson,
    {
        if self.error.is_some() {
            return self;
        }
        for (k, v) in iter {
            if self.member(k, v).error.is_some() {
                break;
            }
        }
        self
    }

    pub fn finish(&mut self) -> std::fmt::Result {
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        write!(self.inner, "}}")?;
        Ok(())
    }
}
