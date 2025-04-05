use crate::JsonFormatter;

pub trait DisplayJson {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result;
}

impl<T: DisplayJson + ?Sized> DisplayJson for &T {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        (*self).fmt(f)
    }
}

impl<T: DisplayJson + ?Sized> DisplayJson for Box<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: DisplayJson> DisplayJson for Option<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        if let Some(v) = self {
            v.fmt(f)
        } else {
            write!(f.inner_mut(), "null")
        }
    }
}

impl DisplayJson for bool {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for i8 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for i16 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for i32 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for i64 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for i128 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for isize {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for u8 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for u16 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for u32 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for u64 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for u128 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for usize {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroI8 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroI16 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroI32 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroI64 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroI128 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroIsize {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroU8 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroU16 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroU32 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroU64 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroU128 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for std::num::NonZeroUsize {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        write!(f.inner_mut(), "{}", self)
    }
}

impl DisplayJson for f32 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        self.is_finite().then_some(self).fmt(f)
    }
}

impl DisplayJson for f64 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        self.is_finite().then_some(self).fmt(f)
    }
}

impl DisplayJson for &str {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self)
    }
}

impl DisplayJson for String {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self)
    }
}

impl<T: DisplayJson, const N: usize> DisplayJson for [T; N] {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.array(|f| f.elements(self.iter()))
    }
}

impl<T: DisplayJson> DisplayJson for &[T] {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.array(|f| f.elements(self.iter()))
    }
}

impl<T: DisplayJson> DisplayJson for Vec<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.array(|f| f.elements(self.iter()))
    }
}

impl<T: DisplayJson> DisplayJson for std::collections::VecDeque<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.array(|f| f.elements(self.iter()))
    }
}

impl<T: DisplayJson> DisplayJson for std::collections::BTreeSet<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.array(|f| f.elements(self.iter()))
    }
}

impl<T: DisplayJson> DisplayJson for std::collections::HashSet<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.array(|f| f.elements(self.iter()))
    }
}
