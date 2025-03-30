use std::{borrow::Cow, ops::Range, str::FromStr};

use crate::{JsonValueKind, parser::JsonParser};

#[derive(Debug)]
#[non_exhaustive]
pub enum JsonParseError {
    UnexpectedEos {
        position: usize,
    },
    // TODO: rename
    NotEos {
        position: usize,
    },
    // TODO: remove?
    UnmatchedArrayClose {
        position: usize,
    },
    UnmatchedObjectClose {
        position: usize,
    },
    InvalidValue {
        position: usize,
    },
    InvalidNumber {
        position: usize,
        // TODO: error_position? or range
    },
    InvalidString {
        position: usize,
        // TODO: error_position? or range
    },
    InvalidArray {
        position: usize,
        // TODO: error_position? or range
    },
    InvalidObject {
        position: usize,
        // TODO: error_position? or range
    },
    UnexpectedKind {
        expected_kinds: &'static [JsonValueKind],
        actual_kind: JsonValueKind,
        position: usize, // TODO: range
    },
    // Valid JSON value, but the content was unexpected.
    UnexpectedValue {
        kind: JsonValueKind,
        position: usize,
        error: Box<dyn Send + Sync + std::error::Error>,
    },
    UnexpectedArraySize {
        expected: usize,
        actual: usize,
        position: usize,
    },
    MissingRequiredMember {
        member_names: Vec<String>,
        position: usize,
    },
    Other {
        position: usize,
        error: Box<dyn Send + Sync + std::error::Error>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JsonValueIndexEntry {
    pub kind: JsonValueKind,
    pub escaped: bool,
    pub text: Range<usize>,
    pub end_index: usize,
}

#[derive(Debug)]
pub struct JsonTextStr<'a> {
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonTextStr<'a> {
    pub fn parse(text: &'a str) -> Result<Self, JsonParseError> {
        let mut parser = JsonParser::new(text);
        parser.parse_value()?;
        parser.check_eos()?;
        Ok(Self {
            text,
            values: parser.values,
        })
    }

    pub fn value(&self) -> JsonValueStr {
        JsonValueStr {
            json: self,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JsonValueStr<'a> {
    json: &'a JsonTextStr<'a>,
    index: usize,
}

impl<'a> JsonValueStr<'a> {
    pub fn kind(self) -> JsonValueKind {
        self.json.values[self.index].kind
    }

    fn entry(&self) -> &JsonValueIndexEntry {
        &self.json.values[self.index]
    }

    pub fn text(self) -> &'a str {
        let text = &self.json.values[self.index].text;
        &self.json.text[text.start..text.end]
    }

    pub fn position(self) -> usize {
        self.json.values[self.index].text.start
    }

    pub fn to_unquoted_str(self) -> Cow<'a, str> {
        if self.entry().escaped {
            let mut unescaped = String::with_capacity(self.text().len());
            let mut chars = self.text().chars();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => {
                        let c = chars.next().expect("infallible");
                        match c {
                            '\\' | '/' | '"' | 'n' | 't' | 'r' | 'b' | 'f' => unescaped.push(c),
                            'u' => {
                                let c = std::str::from_utf8(&[
                                    chars.next().expect("infallible") as u8,
                                    chars.next().expect("infallible") as u8,
                                    chars.next().expect("infallible") as u8,
                                    chars.next().expect("infallible") as u8,
                                ])
                                .ok()
                                .and_then(|code| u32::from_str_radix(code, 16).ok())
                                .and_then(char::from_u32)
                                .expect("infallible");
                                unescaped.push(c);
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unescaped.push(c),
                }
            }
            Cow::Owned(unescaped)
        } else {
            Cow::Borrowed(self.text())
        }
    }

    pub fn non_null_then<F, T>(self, f: F) -> Option<T>
    where
        F: FnOnce(Self) -> T,
    {
        (self.kind() != JsonValueKind::Null).then(|| f(self))
    }

    pub fn non_null_then_try<F, T, E>(self, f: F) -> Result<Option<T>, E>
    where
        F: FnOnce(Self) -> Result<T, E>,
    {
        self.non_null_then(f).transpose()
    }

    pub fn parse<T>(self) -> Result<T, JsonParseError>
    where
        T: FromStr,
        T::Err: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        self.parse_with(|text| text.parse())
    }

    pub fn parse_with<F, T, E>(self, f: F) -> Result<T, JsonParseError>
    where
        F: FnOnce(&str) -> Result<T, E>,
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        f(&self.to_unquoted_str()).map_err(|e| JsonParseError::UnexpectedValue {
            kind: self.kind(),
            position: self.position(),
            error: e.into(),
        })
    }

    pub fn expect(self, kinds: &'static [JsonValueKind]) -> Result<Self, JsonParseError> {
        if kinds.contains(&self.kind()) {
            Ok(self)
        } else {
            Err(JsonParseError::UnexpectedKind {
                expected_kinds: kinds,
                actual_kind: self.kind(),
                position: self.position(),
            })
        }
    }

    pub fn as_bool(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::Bool])
    }

    pub fn as_integer(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::Integer])
    }

    pub fn as_number(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::Integer, JsonValueKind::Float])
    }

    pub fn as_string(self) -> Result<Self, JsonParseError> {
        self.expect(&[JsonValueKind::String])
    }

    pub fn to_array_values(self) -> Result<impl Iterator<Item = JsonValueStr<'a>>, JsonParseError> {
        self.expect(&[JsonValueKind::Array]).map(JsonValues::new)
    }

    pub fn to_fixed_array<const N: usize>(self) -> Result<[JsonValueStr<'a>; N], JsonParseError> {
        let mut values = self.to_array_values()?;
        let mut fixed_array = [self; N];
        for (i, v) in fixed_array.iter_mut().enumerate() {
            *v = values
                .next()
                .ok_or_else(|| JsonParseError::UnexpectedArraySize {
                    expected: N,
                    actual: i,
                    position: self.position(),
                })?;
        }

        let extra = values.count();
        if extra > 0 {
            return Err(JsonParseError::UnexpectedArraySize {
                expected: N,
                actual: N + extra,
                position: self.position(),
            });
        }

        Ok(fixed_array)
    }

    pub fn to_object_members(
        self,
    ) -> Result<impl Iterator<Item = (JsonValueStr<'a>, JsonValueStr<'a>)>, JsonParseError> {
        self.expect(&[JsonValueKind::Object])
            .map(JsonKeyValuePairs::new)
    }

    pub fn to_fixed_object<const N: usize, const M: usize>(
        self,
        required_member_names: [&str; N],
        optional_member_names: [&str; M],
    ) -> Result<([JsonValueStr<'a>; N], [Option<JsonValueStr<'a>>; M]), JsonParseError> {
        let mut required = [self; N];
        let mut optional = [None; M];
        for (k, v) in self.to_object_members()? {
            let k = k.to_unquoted_str();
            if let Some(i) = required_member_names.iter().position(|n| k == *n) {
                required[i] = v;
            } else if let Some(i) = optional_member_names.iter().position(|n| k == *n) {
                optional[i] = Some(v);
            }
        }

        let missing_members = required_member_names
            .iter()
            .zip(required.iter())
            .filter(|(_, value)| value.index != self.index)
            .map(|(&name, _)| name.to_owned())
            .collect::<Vec<_>>();
        if !missing_members.is_empty() {
            return Err(JsonParseError::MissingRequiredMember {
                member_names: missing_members,
                position: self.position(),
            });
        }

        Ok((required, optional))
    }
}

#[derive(Debug)]
struct JsonValues<'a> {
    value: JsonValueStr<'a>,
    end_index: usize,
}

impl<'a> JsonValues<'a> {
    fn new(mut value: JsonValueStr<'a>) -> Self {
        let end_index = value.entry().end_index;
        value.index += 1;
        Self { value, end_index }
    }
}

impl<'a> Iterator for JsonValues<'a> {
    type Item = JsonValueStr<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value.index == self.end_index {
            return None;
        }
        let value = self.value;
        self.value.index = value.entry().end_index;
        Some(value)
    }
}

#[derive(Debug)]
struct JsonKeyValuePairs<'a> {
    inner: JsonValues<'a>,
}

impl<'a> JsonKeyValuePairs<'a> {
    fn new(object: JsonValueStr<'a>) -> Self {
        Self {
            inner: JsonValues::new(object),
        }
    }
}

impl<'a> Iterator for JsonKeyValuePairs<'a> {
    type Item = (JsonValueStr<'a>, JsonValueStr<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.next()?;
        let value = self.inner.next().expect("infallible");
        Some((key, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_text() {
        assert!(matches!(
            JsonTextStr::parse(""),
            Err(JsonParseError::UnexpectedEos { position: 0 })
        ));
        assert!(matches!(
            JsonTextStr::parse("    "),
            Err(JsonParseError::UnexpectedEos { position: 4 })
        ));
    }

    #[test]
    fn parse_nulls() -> Result<(), JsonParseError> {
        let json = JsonTextStr::parse(" null ")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Null);
        assert_eq!(value.text(), "null");
        assert_eq!(value.position(), 1);

        assert!(matches!(
            JsonTextStr::parse("nul"),
            Err(JsonParseError::InvalidValue { position: 0 })
        ));
        assert!(matches!(
            JsonTextStr::parse("nulla"),
            Err(JsonParseError::NotEos { position: 4 })
        ));

        Ok(())
    }

    #[test]
    fn parse_bools() -> Result<(), JsonParseError> {
        let json = JsonTextStr::parse("true")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Bool);
        assert_eq!(value.text(), "true");
        assert_eq!(value.position(), 0);

        let json = JsonTextStr::parse(" false ")?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Bool);
        assert_eq!(value.text(), "false");
        assert_eq!(value.position(), 1);

        assert!(matches!(
            JsonTextStr::parse("false true"),
            Err(JsonParseError::NotEos { position: 6 })
        ));

        Ok(())
    }

    #[test]
    fn parse_numbers() -> Result<(), JsonParseError> {
        // Integers.
        for text in ["0", "-12"] {
            let json = JsonTextStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueKind::Integer);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Floats.
        for text in ["12.3", "12.3e4", "12.3e-4", "-0.3e+4", "12E034"] {
            let json = JsonTextStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueKind::Float);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Invalid nubmers.
        for text in [
            "--1", "+2", "0123", "00", ".123", "1..2", "1ee2", "1e+-3", "123.4.5",
        ] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::InvalidNumber { position: 0 })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        // Invalid values.
        for text in ["e123"] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::InvalidValue { position: 0 })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        // Unexpected EOS.
        for text in ["123.", "-", "123e", "123e-"] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        Ok(())
    }

    #[test]
    fn parse_strings() -> Result<(), JsonParseError> {
        // Non-escaped strings.
        for text in [r#" "" "#, r#" "abc" "#] {
            let json = JsonTextStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueKind::String);
            assert_eq!(value.text(), text.trim());
            assert_eq!(value.position(), 1);
            assert!(!value.entry().escaped);
        }

        // Escaped strings.
        for text in [
            r#" "ab\tc" "#,
            r#" "\n\\a\r\nb\b\"\fc" "#,
            r#" "ab\uF20ac" "#,
        ] {
            let json = JsonTextStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueKind::String);
            assert_eq!(value.text(), text.trim());
            assert_eq!(value.position(), 1);
            assert!(value.entry().escaped);
        }

        // Invalid strings.
        for text in [r#" "ab\xc" "#, r#" "ab\uXyz0c" "#] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::InvalidString { position: 1 })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        // Unexpected EOS.
        for text in [
            r#" "ab "#,
            r#" "ab\"#,
            r#" "ab\u"#,
            r#" "ab\u0"#,
            r#" "ab\u01"#,
            r#" "ab\u012"#,
        ] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        Ok(())
    }

    #[test]
    fn parse_arrays() -> Result<(), JsonParseError> {
        // Arrays.
        for text in [
            "[]",
            "[ \n\t ]",
            "[1  ,null, \"foo\"  ]",
            "[ 1, [[ 2 ], 3,null ],false]",
        ] {
            let json = JsonTextStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueKind::Array);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Invalid arrays.
        for text in ["[,]", "[1,2,]"] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::InvalidArray { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        // Unmatched ']'.
        for text in ["]", "[1,2]]", r#"{"foo":[]]}"#] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::UnmatchedArrayClose { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        // Unexpected EOS.
        for text in ["[", "[1,2", "[1,2,"] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        Ok(())
    }

    #[test]
    fn parse_objects() -> Result<(), JsonParseError> {
        // Objects.
        for text in [
            "{}",
            "{ \n\t }",
            r#"{"foo":1  ,"null": null, "foo" :"bar" }"#,
            r#"{"foo": {}, "bar":[{"a":null}]}"#,
        ] {
            let json = JsonTextStr::parse(text)?;
            let value = json.value();
            assert_eq!(value.kind(), JsonValueKind::Object);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Invalid objects.
        for text in ["{,}", "{:}", r#"{"foo","bar"}"#, r#"{"foo":"bar",}"#] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::InvalidObject { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        // Unmatched '}'.
        for text in ["}", r#"{"1":2}}"#, "[{}}]"] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::UnmatchedObjectClose { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        // Unexpected EOS.
        for text in ["{", r#"{"1" "#, r#"{"1": "#, r#"{"1": 2"#] {
            assert!(
                matches!(
                    JsonTextStr::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonTextStr::parse(text)
            );
        }

        Ok(())
    }
}
