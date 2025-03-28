use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt::Display,
};

use crate::Json;

pub trait DisplayJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    // TODO
    // fn as_json(&self) -> Json<&Self> {
    //     Json(self)
    // }
}

impl<'a, T: DisplayJson> DisplayJson for &'a T {
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

impl DisplayJson for String {
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

impl<T: DisplayJson> DisplayJson for &[T] {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Array(self.iter()).fmt(f)
    }
}

impl<T: DisplayJson> DisplayJson for Vec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Array(self.iter()).fmt(f)
    }
}

impl<T: DisplayJson> DisplayJson for VecDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Array(self.iter()).fmt(f)
    }
}

impl<K: Display, V: DisplayJson> DisplayJson for BTreeMap<K, V> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<K: Display, V: DisplayJson> DisplayJson for HashMap<K, V> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// TODO: ArrayIter?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T>(pub T);

impl<T> DisplayJson for Array<T>
where
    T: Iterator + Clone,
    T::Item: DisplayJson,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut vs = self.0.clone();
        if let Some(v) = vs.next() {
            write!(f, "{}", Json(v))?;
        }
        for v in vs {
            write!(f, ",{}", Json(v))?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

// TODO: ObjectIter
