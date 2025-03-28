use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::Json;

pub trait DisplayJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    // TODO
    // fn as_json(&self) -> Json<&Self> {
    //     Json(self)
    // }
}

pub trait DisplayJsonString: DisplayJson {}

impl<'a, T: DisplayJson> DisplayJson for &'a T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*self).fmt(f)
    }
}

impl<T: DisplayJson> DisplayJsonString for Box<T> {}

impl<'a, T: DisplayJson> DisplayJsonString for &'a T {}

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

impl<'a> DisplayJson for &'a str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        let mut chars = self.chars();
        while let Some(c) = chars.next() {
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

impl DisplayJsonString for String {}

impl<T: DisplayJson> DisplayJson for &[T] {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<T: DisplayJson, const N: usize> DisplayJson for [T; N] {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<T: DisplayJson> DisplayJson for Vec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<T: DisplayJson> DisplayJson for VecDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ArrayFormatter::new(f).values(self.iter()).finish()
    }
}

impl<K: DisplayJsonString, V: DisplayJson> DisplayJson for BTreeMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ObjectFormatter::new(f).members(self.iter()).finish()
    }
}

impl<K: DisplayJsonString, V: DisplayJson> DisplayJson for HashMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ObjectFormatter::new(f).members(self.iter()).finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayIter<T>(pub T);

impl<F, I> DisplayJson for ArrayIter<F>
where
    F: Fn() -> I,
    I: Iterator,
    I::Item: DisplayJson,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ArrayFormatter::new(f).values(self.0()).finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectIter<T>(pub T);

impl<F, I, K, V> DisplayJson for ObjectIter<F>
where
    F: Fn() -> I,
    I: Iterator<Item = (K, V)>,
    K: DisplayJsonString,
    V: DisplayJson,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ObjectFormatter::new(f).members(self.0()).finish()
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

pub struct ArrayFormatter<'a, 'b> {
    inner: &'a mut std::fmt::Formatter<'b>,
    first: bool,
    error: Option<std::fmt::Error>,
}

impl<'a, 'b> ArrayFormatter<'a, 'b> {
    pub fn new(inner: &'a mut std::fmt::Formatter<'b>) -> Self {
        let error = write!(inner, "[").err();
        Self {
            inner,
            first: true,
            error,
        }
    }

    pub fn value<T>(&mut self, value: T) -> &mut Self
    where
        T: DisplayJson,
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

        self.error = write!(self.inner, "{}", Json(value)).err();
        if self.error.is_some() {
            return self;
        }

        self
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

pub struct ObjectFormatter<'a, 'b> {
    inner: &'a mut std::fmt::Formatter<'b>,
    first: bool,
    error: Option<std::fmt::Error>,
}

impl<'a, 'b> ObjectFormatter<'a, 'b> {
    pub fn new(inner: &'a mut std::fmt::Formatter<'b>) -> Self {
        let error = write!(inner, "{{").err();
        Self {
            inner,
            first: true,
            error,
        }
    }

    pub fn member<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: DisplayJsonString,
        V: DisplayJson,
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

        self.error = write!(self.inner, "{}:{}", Json(key), Json(value)).err();
        if self.error.is_some() {
            return self;
        }

        self
    }

    pub fn members<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: DisplayJsonString,
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
