use std::convert::TryFrom;
use std::str::FromStr;

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
    T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
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

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroI8 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroU8 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroI16 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroU16 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroI32 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroU32 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroI64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroU64 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroI128 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroU128 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroIsize {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::num::NonZeroUsize {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        parse_integer(value)
    }
}

fn parse_float<T>(value: RawJsonValue<'_, '_>) -> Result<T, JsonParseError>
where
    T: FromStr,
    T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
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

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for String {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::borrow::Cow<'text, str> {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value.to_unquoted_string_str()
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::path::PathBuf {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let path = value.to_unquoted_string_str()?.into_owned();
        Ok(std::path::PathBuf::from(path))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::IpAddr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::Ipv4Addr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::Ipv6Addr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::SocketAddr {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::SocketAddrV4 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for std::net::SocketAddrV6 {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_unquoted_string_str()?
            .parse()
            .map_err(|e| value.invalid(e))
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

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for Vec<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_array()?
            .map(|v| T::try_from(v).map_err(JsonParseError::from))
            .collect()
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for std::collections::VecDeque<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_array()?
            .map(|v| T::try_from(v).map_err(JsonParseError::from))
            .collect()
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for std::collections::BTreeSet<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>> + Ord,
    JsonParseError: From<T::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_array()?
            .map(|v| T::try_from(v).map_err(JsonParseError::from))
            .collect()
    }
}

impl<'text, 'raw, T> TryFrom<RawJsonValue<'text, 'raw>> for std::collections::HashSet<T>
where
    T: TryFrom<RawJsonValue<'text, 'raw>> + Eq + std::hash::Hash,
    JsonParseError: From<T::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        value
            .to_array()?
            .map(|v| T::try_from(v).map_err(JsonParseError::from))
            .collect()
    }
}

impl<'text, 'raw, T, const N: usize> TryFrom<RawJsonValue<'text, 'raw>> for [T; N]
where
    T: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let mut values = value.to_array()?;
        let mut fixed_array = [value; N];
        for (i, v) in fixed_array.iter_mut().enumerate() {
            *v = values.next().ok_or_else(|| {
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

        let mut results = fixed_array.map(|v| T::try_from(v).map_err(Some));
        for result in &mut results {
            if let Err(e) = result {
                return Err(e.take().expect("infallible").into());
            }
        }
        Ok(results.map(|r| r.ok().expect("infallible")))
    }
}

impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for () {
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let []: [RawJsonValue<'text, 'raw>; 0] = value.try_into()?;
        Ok(())
    }
}

impl<'text, 'raw, T0> TryFrom<RawJsonValue<'text, 'raw>> for (T0,)
where
    //T0: TryFrom<RawJsonValue<'text, 'raw>, Error = JsonParseError>,
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T0::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0] = <[RawJsonValue; 1]>::try_from(value)?;
        Ok((T0::try_from(v0).map_err(JsonParseError::from)?,))
    }
}

impl<'text, 'raw, T0, T1> TryFrom<RawJsonValue<'text, 'raw>> for (T0, T1)
where
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    T1: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T0::Error> + From<T1::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0, v1]: [RawJsonValue<'text, 'raw>; 2] = value.try_into()?;
        Ok((
            T0::try_from(v0).map_err(JsonParseError::from)?,
            T1::try_from(v1).map_err(JsonParseError::from)?,
        ))
    }
}

impl<'text, 'raw, T0, T1, T2> TryFrom<RawJsonValue<'text, 'raw>> for (T0, T1, T2)
where
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    T1: TryFrom<RawJsonValue<'text, 'raw>>,
    T2: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T0::Error> + From<T1::Error> + From<T2::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0, v1, v2]: [RawJsonValue<'text, 'raw>; 3] = value.try_into()?;
        Ok((
            T0::try_from(v0).map_err(JsonParseError::from)?,
            T1::try_from(v1).map_err(JsonParseError::from)?,
            T2::try_from(v2).map_err(JsonParseError::from)?,
        ))
    }
}

impl<'text, 'raw, T0, T1, T2, T3> TryFrom<RawJsonValue<'text, 'raw>> for (T0, T1, T2, T3)
where
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    T1: TryFrom<RawJsonValue<'text, 'raw>>,
    T2: TryFrom<RawJsonValue<'text, 'raw>>,
    T3: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T0::Error> + From<T1::Error> + From<T2::Error> + From<T3::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0, v1, v2, v3]: [RawJsonValue<'text, 'raw>; 4] = value.try_into()?;
        Ok((
            T0::try_from(v0).map_err(JsonParseError::from)?,
            T1::try_from(v1).map_err(JsonParseError::from)?,
            T2::try_from(v2).map_err(JsonParseError::from)?,
            T3::try_from(v3).map_err(JsonParseError::from)?,
        ))
    }
}

impl<'text, 'raw, T0, T1, T2, T3, T4> TryFrom<RawJsonValue<'text, 'raw>> for (T0, T1, T2, T3, T4)
where
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    T1: TryFrom<RawJsonValue<'text, 'raw>>,
    T2: TryFrom<RawJsonValue<'text, 'raw>>,
    T3: TryFrom<RawJsonValue<'text, 'raw>>,
    T4: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError:
        From<T0::Error> + From<T1::Error> + From<T2::Error> + From<T3::Error> + From<T4::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0, v1, v2, v3, v4]: [RawJsonValue<'text, 'raw>; 5] = value.try_into()?;
        Ok((
            T0::try_from(v0).map_err(JsonParseError::from)?,
            T1::try_from(v1).map_err(JsonParseError::from)?,
            T2::try_from(v2).map_err(JsonParseError::from)?,
            T3::try_from(v3).map_err(JsonParseError::from)?,
            T4::try_from(v4).map_err(JsonParseError::from)?,
        ))
    }
}

impl<'text, 'raw, T0, T1, T2, T3, T4, T5> TryFrom<RawJsonValue<'text, 'raw>>
    for (T0, T1, T2, T3, T4, T5)
where
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    T1: TryFrom<RawJsonValue<'text, 'raw>>,
    T2: TryFrom<RawJsonValue<'text, 'raw>>,
    T3: TryFrom<RawJsonValue<'text, 'raw>>,
    T4: TryFrom<RawJsonValue<'text, 'raw>>,
    T5: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T0::Error>
        + From<T1::Error>
        + From<T2::Error>
        + From<T3::Error>
        + From<T4::Error>
        + From<T5::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0, v1, v2, v3, v4, v5]: [RawJsonValue<'text, 'raw>; 6] = value.try_into()?;
        Ok((
            T0::try_from(v0).map_err(JsonParseError::from)?,
            T1::try_from(v1).map_err(JsonParseError::from)?,
            T2::try_from(v2).map_err(JsonParseError::from)?,
            T3::try_from(v3).map_err(JsonParseError::from)?,
            T4::try_from(v4).map_err(JsonParseError::from)?,
            T5::try_from(v5).map_err(JsonParseError::from)?,
        ))
    }
}

impl<'text, 'raw, T0, T1, T2, T3, T4, T5, T6> TryFrom<RawJsonValue<'text, 'raw>>
    for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    T1: TryFrom<RawJsonValue<'text, 'raw>>,
    T2: TryFrom<RawJsonValue<'text, 'raw>>,
    T3: TryFrom<RawJsonValue<'text, 'raw>>,
    T4: TryFrom<RawJsonValue<'text, 'raw>>,
    T5: TryFrom<RawJsonValue<'text, 'raw>>,
    T6: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T0::Error>
        + From<T1::Error>
        + From<T2::Error>
        + From<T3::Error>
        + From<T4::Error>
        + From<T5::Error>
        + From<T6::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0, v1, v2, v3, v4, v5, v6]: [RawJsonValue<'text, 'raw>; 7] = value.try_into()?;
        Ok((
            T0::try_from(v0).map_err(JsonParseError::from)?,
            T1::try_from(v1).map_err(JsonParseError::from)?,
            T2::try_from(v2).map_err(JsonParseError::from)?,
            T3::try_from(v3).map_err(JsonParseError::from)?,
            T4::try_from(v4).map_err(JsonParseError::from)?,
            T5::try_from(v5).map_err(JsonParseError::from)?,
            T6::try_from(v6).map_err(JsonParseError::from)?,
        ))
    }
}

impl<'text, 'raw, T0, T1, T2, T3, T4, T5, T6, T7> TryFrom<RawJsonValue<'text, 'raw>>
    for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: TryFrom<RawJsonValue<'text, 'raw>>,
    T1: TryFrom<RawJsonValue<'text, 'raw>>,
    T2: TryFrom<RawJsonValue<'text, 'raw>>,
    T3: TryFrom<RawJsonValue<'text, 'raw>>,
    T4: TryFrom<RawJsonValue<'text, 'raw>>,
    T5: TryFrom<RawJsonValue<'text, 'raw>>,
    T6: TryFrom<RawJsonValue<'text, 'raw>>,
    T7: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<T0::Error>
        + From<T1::Error>
        + From<T2::Error>
        + From<T3::Error>
        + From<T4::Error>
        + From<T5::Error>
        + From<T6::Error>
        + From<T7::Error>,
{
    type Error = JsonParseError;

    fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
        let [v0, v1, v2, v3, v4, v5, v6, v7]: [RawJsonValue<'text, 'raw>; 8] = value.try_into()?;
        Ok((
            T0::try_from(v0).map_err(JsonParseError::from)?,
            T1::try_from(v1).map_err(JsonParseError::from)?,
            T2::try_from(v2).map_err(JsonParseError::from)?,
            T3::try_from(v3).map_err(JsonParseError::from)?,
            T4::try_from(v4).map_err(JsonParseError::from)?,
            T5::try_from(v5).map_err(JsonParseError::from)?,
            T6::try_from(v6).map_err(JsonParseError::from)?,
            T7::try_from(v7).map_err(JsonParseError::from)?,
        ))
    }
}

impl<'text, 'raw, K, V> TryFrom<RawJsonValue<'text, 'raw>> for std::collections::BTreeMap<K, V>
where
    K: FromStr + Ord,
    K::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    V: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<V::Error>,
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
                    V::try_from(v).map_err(JsonParseError::from)?,
                ))
            })
            .collect()
    }
}

impl<'text, 'raw, K, V> TryFrom<RawJsonValue<'text, 'raw>> for std::collections::HashMap<K, V>
where
    K: FromStr + Eq + std::hash::Hash,
    K::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    V: TryFrom<RawJsonValue<'text, 'raw>>,
    JsonParseError: From<V::Error>,
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
                    V::try_from(v).map_err(JsonParseError::from)?,
                ))
            })
            .collect()
    }
}
