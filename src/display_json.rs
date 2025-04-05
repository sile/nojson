use crate::JsonFormatter;

pub trait DisplayJson {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result;
}

impl<T: DisplayJson> DisplayJson for &T {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        (*self).fmt(f)
    }
}

impl<T: DisplayJson> DisplayJson for Box<T> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl DisplayJson for bool {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.write_value(self)
    }
}

impl DisplayJson for i8 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.write_value(self)
    }
}
