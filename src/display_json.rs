use std::fmt::Display;

use crate::JsonFormatter;

/// A variant of the [`Display`] trait for JSON.
///
/// This trait allows Rust types to be formatted as valid JSON.
/// Unlike the standard [`Display`] trait, [`DisplayJson`] is designed for
/// JSON serialization and supports proper escaping,
/// indentation, and other JSON-specific formatting features.
///
/// # Implementation Notes
///
/// `nojson` provides built-in implementations for many common Rust types:
/// - Basic types (booleans, integers, floats, strings)
/// - Collection types (arrays, vectors, sets, maps)
/// - Nullable types (via `Option<T>`)
/// - Reference types
///
/// # Examples
///
/// Implementing `DisplayJson` for a struct:
/// ```
/// use nojson::{DisplayJson, JsonFormatter, Json};
///
/// struct Person {
///     name: String,
///     age: u32,
///     email: Option<String>,
/// }
///
/// impl DisplayJson for Person {
///     fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
///         f.object(|f| {
///             f.member("name", &self.name)?;
///             f.member("age", &self.age)?;
///             f.member("email", &self.email)
///         })
///     }
/// }
///
/// // Now you can use it with `Json` wrapper
/// let person = Person {
///     name: "Alice".to_string(),
///     age: 30,
///     email: Some("alice@example.com".to_string()),
/// };
///
/// assert_eq!(
///     Json(&person).to_string(),
///     r#"{"name":"Alice","age":30,"email":"alice@example.com"}"#
/// );
/// ```
///
/// Generating JSON in-place using [`json()`](crate::json):
/// ```
/// use nojson::{DisplayJson, json};
/// use std::collections::BTreeMap;
///
/// // Build a JSON object with pretty-printing.
/// let object = json(|f| {
///     f.set_indent_size(2);
///     f.set_spacing(true);
///     f.object(|f| {
///         f.member("name", "Example")?;
///         f.member("counts", &[1, 2, 3])?;
///         f.member("config", json(|f| f.object(|f| {
///             f.member("enabled", true);
///             f.member("visible", false)
///         })))
///     })
/// });
///
/// // Generate a JSON text from the object.
/// let text = format!("\n{}", object);
/// assert_eq!(text, r#"
/// {
///   "name": "Example",
///   "counts": [
///     1,
///     2,
///     3
///   ],
///   "config": {
///     "enabled": true,
///     "visible": false
///   }
/// }"#);
/// ```
pub trait DisplayJson {
    /// Formats the value as JSON into the provided formatter.
    ///
    /// This method is similar to [`Display::fmt()`], but accepts a
    /// [`JsonFormatter`] which provides additional methods for JSON-specific formatting.
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result;
}

impl<T: DisplayJson + ?Sized> DisplayJson for &T {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: DisplayJson + ?Sized> DisplayJson for &mut T {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        (**self).fmt(f)
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
        if self.is_finite() {
            write!(f.inner_mut(), "{}", self)
        } else {
            write!(f.inner_mut(), "null")
        }
    }
}

impl DisplayJson for f64 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        if self.is_finite() {
            write!(f.inner_mut(), "{}", self)
        } else {
            write!(f.inner_mut(), "null")
        }
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

impl DisplayJson for &std::path::Path {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self.display())
    }
}

impl DisplayJson for std::path::PathBuf {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self.display())
    }
}

impl DisplayJson for std::net::SocketAddr {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self)
    }
}

impl DisplayJson for std::net::SocketAddrV4 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self)
    }
}

impl DisplayJson for std::net::SocketAddrV6 {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self)
    }
}

impl DisplayJson for std::net::IpAddr {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self)
    }
}

impl DisplayJson for std::net::Ipv4Addr {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.string(self)
    }
}

impl DisplayJson for std::net::Ipv6Addr {
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

impl<K: Display, V: DisplayJson> DisplayJson for std::collections::BTreeMap<K, V> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.object(|f| f.members(self.iter()))
    }
}

impl<K: Display, V: DisplayJson> DisplayJson for std::collections::HashMap<K, V> {
    fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> std::fmt::Result {
        f.object(|f| f.members(self.iter()))
    }
}
