use alloc::{borrow::Cow, boxed::Box, format, rc::Rc, string::String, sync::Arc, vec::Vec};
use core::str::FromStr;

use crate::{JsonParseError, RawJsonValue};

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for bool {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .as_boolean_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

fn parse_integer<T>(value: RawJsonValue<'_, '_>) -> Result<T, JsonParseError>
where
    T: FromStr,
    T::Err: Into<Box<dyn Send + Sync + core::error::Error>>,
{
    value
        .as_integer_str()?
        .parse()
        .map_err(|e| value.invalid(e))
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for i8 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for u8 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for i16 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for u16 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for i32 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for u32 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for i64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for u64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for i128 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for u128 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for isize {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for usize {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroI8 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroU8 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroI16 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroU16 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroI32 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroU32 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroI64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroU64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroI128 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroU128 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroIsize {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for core::num::NonZeroUsize {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

fn parse_float<T>(value: RawJsonValue<'_, '_>) -> Result<T, JsonParseError>
where
    T: FromStr,
    T::Err: Into<Box<dyn Send + Sync + core::error::Error>>,
{
    value.as_number_str()?.parse().map_err(|e| value.invalid(e))
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for f32 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_float(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for f64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_float(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for char {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let s = value.to_unquoted_string_str()?;
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            (Some(_), Some(_)) => Err(value.invalid(
                "expected a string with exactly one character, but got multiple characters",
            )),
            _ => Err(value
                .invalid("expected a string with exactly one character, but got an empty string")),
        }
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for &'text str {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value.as_string_str()
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for String {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for Cow<'text, str> {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value.to_unquoted_string_str()
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::path::PathBuf {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let path = value.to_unquoted_string_str()?.into_owned();
        Ok(std::path::PathBuf::from(path))
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::IpAddr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::Ipv4Addr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::Ipv6Addr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::SocketAddr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::SocketAddrV4 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::SocketAddrV6 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for Rc<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        T::try_from(value).map(Rc::new)
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for Arc<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        T::try_from(value).map(Arc::new)
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for Option<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        if value.kind().is_null() {
            Ok(None)
        } else {
            T::try_from(value).map(Some)
        }
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for Vec<RawJsonValue<'text, 'raw>> {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        Ok(value.to_array()?.collect())
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for Vec<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value.to_array()?.map(|v| T::try_from(v)).collect()
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>>
    for alloc::collections::VecDeque<RawJsonValue<'text, 'raw>>
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        Ok(value.to_array()?.collect())
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for alloc::collections::VecDeque<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value.to_array()?.map(|v| T::try_from(v)).collect()
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for alloc::collections::BTreeSet<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError> + Ord,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value.to_array()?.map(|v| T::try_from(v)).collect()
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for std::collections::HashSet<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError> + Eq + core::hash::Hash,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value.to_array()?.map(|v| T::try_from(v)).collect()
    }
}

impl<'text, 'raw, const N: usize> TryFrom<RawJsonValue<'text, 'raw>>
    for [RawJsonValue<'text, 'raw>; N]
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let mut values = value.to_array()?;
        let mut fixed_array = [value; N];

        for (i, slot) in fixed_array.iter_mut().enumerate() {
            *slot = values.next().ok_or_else(|| {
                value.invalid(format!(
                    "expected an array with {N} elements, but got only {i} elements"
                ))
            })?;
        }

        let extra = values.count();
        if extra > 0 {
            return Err(value.invalid(format!(
                "expected an array with {N} elements, but got {} elements",
                N + extra
            )));
        }

        Ok(fixed_array)
    }
}

impl<'text, 'raw, T, const N: usize> TryFrom<RawJsonValue<'text, 'raw>> for [T; N]
where
    T: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let fixed_array: [RawJsonValue<'text, 'raw>; N] = value.try_into()?;
        let mut results = fixed_array.map(|v| T::try_from(v).map_err(Some));
        for result in &mut results {
            if let Err(e) = result {
                return Err(e.take().expect("infallible"));
            }
        }
        Ok(results.map(|r| r.expect("infallible")))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for () {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        if value.kind().is_null() {
            Ok(())
        } else {
            Err(value.invalid(format!("expected null, but found {:?}", value.kind())))
        }
    }
}

impl<'text, 'raw, K> TryFrom<RawJsonValue<'text, 'raw>>
    for alloc::collections::BTreeMap<K, RawJsonValue<'text, 'raw>>
where
    K: FromStr + Ord,
    K::Err: Into<Box<dyn Send + Sync + core::error::Error>>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_object()?
            .map(|(k, v)| {
                Ok((
                    k.to_unquoted_string_str()?
                        .parse()
                        .map_err(|e| k.invalid(e))?,
                    v,
                ))
            })
            .collect()
    }
}

impl<'text, 'raw, K, V> TryFrom<RawJsonValue<'text, 'raw>> for alloc::collections::BTreeMap<K, V>
where
    K: FromStr + Ord,
    K::Err: Into<Box<dyn Send + Sync + core::error::Error>>,
    V: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_object()?
            .map(|(k, v)| {
                Ok((
                    k.to_unquoted_string_str()?
                        .parse()
                        .map_err(|e| k.invalid(e))?,
                    V::try_from(v)?,
                ))
            })
            .collect()
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw, K> TryFrom<RawJsonValue<'text, 'raw>>
    for std::collections::HashMap<K, RawJsonValue<'text, 'raw>>
where
    K: FromStr + Eq + core::hash::Hash,
    K::Err: Into<Box<dyn Send + Sync + core::error::Error>>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_object()?
            .map(|(k, v)| {
                Ok((
                    k.to_unquoted_string_str()?
                        .parse()
                        .map_err(|e| k.invalid(e))?,
                    v,
                ))
            })
            .collect()
    }
}

#[cfg(feature = "std")]
impl<'text, 'raw, K, V> TryFrom<RawJsonValue<'text, 'raw>> for std::collections::HashMap<K, V>
where
    K: FromStr + Eq + core::hash::Hash,
    K::Err: Into<Box<dyn Send + Sync + core::error::Error>>,
    V: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_object()?
            .map(|(k, v)| {
                Ok((
                    k.to_unquoted_string_str()?
                        .parse()
                        .map_err(|e| k.invalid(e))?,
                    V::try_from(v)?,
                ))
            })
            .collect()
    }
}
