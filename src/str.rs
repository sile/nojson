use std::{borrow::Cow, ops::Range, str::FromStr};

use crate::{JsonValueKind, parser::JsonParser};

pub use crate::parse_error::JsonParseError;

pub trait FromRawJsonValue<'a>: Sized {
    fn from_raw_json_value(raw: RawJsonValue<'a>) -> Result<Self, JsonParseError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JsonValueIndexEntry {
    pub kind: JsonValueKind,
    pub escaped: bool,
    pub text: Range<usize>,
    pub end_index: usize,
}

#[derive(Debug)]
pub struct JsonText<'a> {
    text: &'a str,
    values: Vec<JsonValueIndexEntry>,
}

impl<'a> JsonText<'a> {
    pub fn parse(text: &'a str) -> Result<Self, JsonParseError> {
        let values = JsonParser::new(text).parse()?;
        Ok(Self { text, values })
    }

    pub fn raw_value(&self) -> RawJsonValue {
        RawJsonValue {
            json: self,
            index: 0,
        }
    }

    pub fn get_raw_value_by_position(&self, position: usize) -> Option<RawJsonValue> {
        let mut value = self.raw_value();
        if !value.entry().text.contains(&position) {
            return None;
        }
        while let Some(child) = Children::new(value).find(|c| c.entry().text.contains(&position)) {
            value = child;
        }
        Some(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawJsonValue<'a> {
    json: &'a JsonText<'a>,
    index: usize,
}

impl<'a> RawJsonValue<'a> {
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

    pub fn to_invalid_value_error<E>(self, error: E) -> JsonParseError
    where
        E: Into<Box<dyn Send + Sync + std::error::Error>>,
    {
        JsonParseError::InvalidValue {
            kind: self.kind(),
            position: self.position(),
            error: error.into(),
        }
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
        f(&self.to_unquoted_str()).map_err(|e| JsonParseError::InvalidValue {
            kind: self.kind(),
            position: self.position(),
            error: e.into(),
        })
    }

    pub fn expect(self, kinds: &'static [JsonValueKind]) -> Result<Self, JsonParseError> {
        if kinds.contains(&self.kind()) {
            Ok(self)
        } else {
            Err(self.to_invalid_value_error(format!(
                "expected {:?}, but found {:?}",
                if kinds.len() == 1 {
                    format!("{:?}", kinds[0])
                } else {
                    format!("one of {:?}", kinds)
                },
                self.kind()
            )))
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

    pub fn to_array_values(self) -> Result<impl Iterator<Item = RawJsonValue<'a>>, JsonParseError> {
        self.expect(&[JsonValueKind::Array]).map(Children::new)
    }

    pub fn to_fixed_array<const N: usize>(self) -> Result<[RawJsonValue<'a>; N], JsonParseError> {
        let mut values = self.to_array_values()?;
        let mut fixed_array = [self; N];
        for (i, v) in fixed_array.iter_mut().enumerate() {
            *v = values.next().ok_or_else(|| {
                self.to_invalid_value_error(format!(
                    "expected an array with {N} elements, but got only {i} elements"
                ))
            })?;
        }

        let extra = values.count();
        if extra > 0 {
            return Err(self.to_invalid_value_error(format!(
                "expected an array with {N} elements, but got {} elements",
                N + extra
            )));
        }

        Ok(fixed_array)
    }

    pub fn to_object_members(
        self,
    ) -> Result<impl Iterator<Item = (RawJsonValue<'a>, RawJsonValue<'a>)>, JsonParseError> {
        self.expect(&[JsonValueKind::Object])
            .map(JsonKeyValuePairs::new)
    }

    pub fn to_fixed_object<const N: usize, const M: usize>(
        self,
        required_member_names: [&str; N],
        optional_member_names: [&str; M],
    ) -> Result<([RawJsonValue<'a>; N], [Option<RawJsonValue<'a>>; M]), JsonParseError> {
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

        if required.iter().any(|v| v.index == self.index) {
            let missings = required_member_names
                .iter()
                .zip(required.iter())
                .filter(|(_, value)| value.index != self.index)
                .map(|(name, _)| name)
                .collect::<Vec<_>>();
            return Err(self
                .to_invalid_value_error(format!("missing required object members: {missings:?}")));
        }

        Ok((required, optional))
    }
}

#[derive(Debug)]
struct Children<'a> {
    value: RawJsonValue<'a>,
    end_index: usize,
}

impl<'a> Children<'a> {
    fn new(mut value: RawJsonValue<'a>) -> Self {
        let end_index = value.entry().end_index;
        value.index += 1;
        Self { value, end_index }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = RawJsonValue<'a>;

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
    inner: Children<'a>,
}

impl<'a> JsonKeyValuePairs<'a> {
    fn new(object: RawJsonValue<'a>) -> Self {
        Self {
            inner: Children::new(object),
        }
    }
}

impl<'a> Iterator for JsonKeyValuePairs<'a> {
    type Item = (RawJsonValue<'a>, RawJsonValue<'a>);

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
            JsonText::parse(""),
            Err(JsonParseError::UnexpectedEos {
                kind: None,
                position: 0
            })
        ));
        assert!(matches!(
            JsonText::parse("    "),
            Err(JsonParseError::UnexpectedEos {
                kind: None,
                position: 4
            })
        ));
    }

    #[test]
    fn parse_nulls() -> Result<(), JsonParseError> {
        let json = JsonText::parse(" null ")?;
        let value = json.raw_value();
        assert_eq!(value.kind(), JsonValueKind::Null);
        assert_eq!(value.text(), "null");
        assert_eq!(value.position(), 1);

        assert!(matches!(
            JsonText::parse("nuL"),
            Err(JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::Null),
                position: 2
            })
        ));
        assert!(matches!(
            JsonText::parse("nul"),
            Err(JsonParseError::UnexpectedEos {
                kind: Some(JsonValueKind::Null),
                position: 3
            })
        ));
        assert!(matches!(
            JsonText::parse("nulla"),
            Err(JsonParseError::UnexpectedTrailingChar {
                kind: JsonValueKind::Null,
                position: 4
            })
        ));

        Ok(())
    }

    #[test]
    fn parse_bools() -> Result<(), JsonParseError> {
        let json = JsonText::parse("true")?;
        let value = json.raw_value();
        assert_eq!(value.kind(), JsonValueKind::Bool);
        assert_eq!(value.text(), "true");
        assert_eq!(value.position(), 0);

        let json = JsonText::parse(" false ")?;
        let value = json.raw_value();
        assert_eq!(value.kind(), JsonValueKind::Bool);
        assert_eq!(value.text(), "false");
        assert_eq!(value.position(), 1);

        assert!(matches!(
            JsonText::parse("false true"),
            Err(JsonParseError::UnexpectedTrailingChar {
                kind: JsonValueKind::Bool,
                position: 6
            })
        ));
        assert!(matches!(
            JsonText::parse("fale"),
            Err(JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::Bool),
                position: 3
            })
        ));
        assert!(matches!(
            JsonText::parse("tr"),
            Err(JsonParseError::UnexpectedEos {
                kind: Some(JsonValueKind::Bool),
                position: 2
            })
        ));

        Ok(())
    }

    #[test]
    fn parse_numbers() -> Result<(), JsonParseError> {
        // Integers.
        for text in ["0", "-12"] {
            let json = JsonText::parse(text)?;
            let value = json.raw_value();
            assert_eq!(value.kind(), JsonValueKind::Integer);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Floats.
        for text in ["12.3", "12.3e4", "12.3e-4", "-0.3e+4", "12E034"] {
            let json = JsonText::parse(text)?;
            let value = json.raw_value();
            assert_eq!(value.kind(), JsonValueKind::Float);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Malformed integers.
        for (text, position) in [("--1", 1)] {
            let e = JsonText::parse(text).expect_err("error");
            assert!(
                matches!(
                    e,
                    JsonParseError::UnexpectedValueChar {
                        kind: Some(JsonValueKind::Integer),
                        ..
                    }
                ),
                "text={text}, error={e:?}"
            );
            assert_eq!(e.position(), position);
        }

        // Malformed floats.
        for (text, position) in [("1..2", 2), ("1ee2", 2), ("1e+-3", 3)] {
            let e = JsonText::parse(text).expect_err("error");
            assert!(
                matches!(
                    e,
                    JsonParseError::UnexpectedValueChar {
                        kind: Some(JsonValueKind::Float),
                        ..
                    }
                ),
                "text={text}, error={e:?}"
            );
            assert_eq!(e.position(), position);
        }

        // Malformed values.
        for text in ["e123", "+2", ".123"] {
            assert!(
                matches!(
                    JsonText::parse(text),
                    Err(JsonParseError::UnexpectedValueChar {
                        kind: None,
                        position: 0
                    })
                ),
                "text={text}, error={:?}",
                JsonText::parse(text)
            );
        }

        // Unexpected trailing char.
        for (text, position) in [("123.4.5", 5), ("0123", 1), ("00", 1)] {
            let e = JsonText::parse(text).expect_err("error");
            assert!(
                matches!(
                    e,
                    JsonParseError::UnexpectedTrailingChar {
                        kind: JsonValueKind::Float | JsonValueKind::Integer,
                        ..
                    }
                ),
                "text={text}, error={e:?}"
            );
            assert_eq!(e.position(), position);
        }

        // Unexpected EOS.
        for text in ["123.", "-", "123e", "123e-"] {
            assert!(
                matches!(
                    JsonText::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonText::parse(text)
            );
        }

        Ok(())
    }

    #[test]
    fn parse_strings() -> Result<(), JsonParseError> {
        // Non-escaped strings.
        for text in [r#" "" "#, r#" "abc" "#] {
            let json = JsonText::parse(text)?;
            let value = json.raw_value();
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
            let json = JsonText::parse(text)?;
            let value = json.raw_value();
            assert_eq!(value.kind(), JsonValueKind::String);
            assert_eq!(value.text(), text.trim());
            assert_eq!(value.position(), 1);
            assert!(value.entry().escaped);
        }

        // Malformed strings.
        for (text, error_position) in [(r#" "ab\xc" "#, 5), (r#" "ab\uXyz0c" "#, 6)] {
            let e = JsonText::parse(text).expect_err("error");
            assert!(
                matches!(
                    e,
                    JsonParseError::UnexpectedValueChar {
                        kind: Some(JsonValueKind::String),
                        ..
                    }
                ),
                "text={text}, error={:?}",
                JsonText::parse(text)
            );
            assert_eq!(e.position(), error_position);
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
                    JsonText::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonText::parse(text)
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
            let json = JsonText::parse(text)?;
            let value = json.raw_value();
            assert_eq!(value.kind(), JsonValueKind::Array);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Malformed arrays.
        for (text, position) in [("[,]", 1), ("[1,2,]", 5)] {
            let e = JsonText::parse(text).expect_err("error");
            assert!(
                matches!(
                    e,
                    JsonParseError::UnexpectedValueChar {
                        kind: Some(JsonValueKind::Array),
                        ..
                    }
                ),
                "text={text}, error={:?}",
                JsonText::parse(text)
            );
            assert_eq!(e.position(), position);
        }

        // Unmatched ']'.
        let text = "]";
        let e = JsonText::parse(text).expect_err("error");
        assert!(
            matches!(e, JsonParseError::UnexpectedValueChar { kind: None, .. }),
            "text={text}, error={e:?}",
        );
        assert_eq!(e.position(), 0);

        let text = "[1,2]]";
        let e = JsonText::parse(text).expect_err("error");
        assert!(
            matches!(
                e,
                JsonParseError::UnexpectedTrailingChar {
                    kind: JsonValueKind::Array,
                    position: 5
                }
            ),
            "text={text}, error={e:?}",
        );

        let text = r#"{"foo":[]]}"#;
        let e = JsonText::parse(text).expect_err("error");
        assert!(
            matches!(
                e,
                JsonParseError::UnexpectedValueChar {
                    kind: Some(JsonValueKind::Object),
                    ..
                }
            ),
            "text={text}, error={e:?}",
        );
        assert_eq!(e.position(), 9);

        // Unexpected EOS.
        for text in ["[", "[1,2", "[1,2,"] {
            assert!(
                matches!(
                    JsonText::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonText::parse(text)
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
            let json = JsonText::parse(text)?;
            let value = json.raw_value();
            assert_eq!(value.kind(), JsonValueKind::Object);
            assert_eq!(value.text(), text);
            assert_eq!(value.position(), 0);
        }

        // Malformed objects.
        for (text, position) in [
            ("{,}", 1),
            ("{:}", 1),
            (r#"{"foo","bar"}"#, 6),
            (r#"{"foo":"bar",}"#, 13),
        ] {
            let e = JsonText::parse(text).expect_err("error");
            assert!(
                matches!(
                    e,
                    JsonParseError::UnexpectedValueChar {
                        kind: Some(JsonValueKind::Object),
                        ..
                    }
                ),
                "text={text}, error={e:?}",
            );
            assert_eq!(e.position(), position);
        }

        // Unmatched '}'.
        let text = "}";
        let e = JsonText::parse(text).expect_err("error");
        assert!(
            matches!(e, JsonParseError::UnexpectedValueChar { kind: None, .. }),
            "text={text}, error={e:?}",
        );
        assert_eq!(e.position(), 0);

        let text = r#"{"1":2}}"#;
        let e = JsonText::parse(text).expect_err("error");
        assert!(
            matches!(
                e,
                JsonParseError::UnexpectedTrailingChar {
                    kind: JsonValueKind::Object,
                    position: 7
                }
            ),
            "text={text}, error={e:?}",
        );

        let text = "[{}}]";
        let e = JsonText::parse(text).expect_err("error");
        assert!(
            matches!(
                e,
                JsonParseError::UnexpectedValueChar {
                    kind: Some(JsonValueKind::Array),
                    ..
                }
            ),
            "text={text}, error={e:?}",
        );
        assert_eq!(e.position(), 3);

        // Unexpected EOS.
        for text in ["{", r#"{"1" "#, r#"{"1": "#, r#"{"1": 2"#] {
            assert!(
                matches!(
                    JsonText::parse(text),
                    Err(JsonParseError::UnexpectedEos { .. })
                ),
                "text={text}, error={:?}",
                JsonText::parse(text)
            );
        }

        Ok(())
    }
}
