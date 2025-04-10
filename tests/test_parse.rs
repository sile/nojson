use std::{borrow::Cow, collections::BTreeMap};

use nojson::{FromRawJsonValue, Json, JsonParseError, JsonValueKind, RawJson, RawJsonValue};

macro_rules! assert_parse_error_matches {
    ($text:expr, $error_pattern:pat) => {{
        let text = $text;
        let e = RawJson::parse(text).expect_err("expected parsing to fail");
        assert!(matches!(e, $error_pattern), "text={text}, error={e:?}");
        e
    }};
}

#[test]
fn parse_empty_text() {
    assert_parse_error_matches!(
        "",
        JsonParseError::UnexpectedEos {
            kind: None,
            position: 0
        }
    );
    assert_parse_error_matches!(
        "    ",
        JsonParseError::UnexpectedEos {
            kind: None,
            position: 4
        }
    );
}

#[test]
fn parse_nulls() -> Result<(), JsonParseError> {
    let json = RawJson::parse(" null ")?;
    let value = json.value();
    assert_eq!(value.kind(), JsonValueKind::Null);
    assert_eq!(value.as_raw_str(), "null");
    assert_eq!(value.position(), 1);

    assert_parse_error_matches!(
        "nuL",
        JsonParseError::UnexpectedValueChar {
            kind: Some(JsonValueKind::Null),
            position: 2
        }
    );
    assert_parse_error_matches!(
        "nul",
        JsonParseError::UnexpectedEos {
            kind: Some(JsonValueKind::Null),
            position: 3
        }
    );
    assert_parse_error_matches!(
        "nulla",
        JsonParseError::UnexpectedTrailingChar {
            kind: JsonValueKind::Null,
            position: 4
        }
    );

    Ok(())
}

#[test]
fn parse_bools() -> Result<(), JsonParseError> {
    let json = RawJson::parse("true")?;
    let value = json.value();
    assert_eq!(value.kind(), JsonValueKind::Boolean);
    assert_eq!(value.as_raw_str(), "true");
    assert_eq!(value.position(), 0);

    let json = RawJson::parse(" false ")?;
    let value = json.value();
    assert_eq!(value.kind(), JsonValueKind::Boolean);
    assert_eq!(value.as_raw_str(), "false");
    assert_eq!(value.position(), 1);

    assert_parse_error_matches!(
        "false true",
        JsonParseError::UnexpectedTrailingChar {
            kind: JsonValueKind::Boolean,
            position: 6
        }
    );
    assert_parse_error_matches!(
        "fale",
        JsonParseError::UnexpectedValueChar {
            kind: Some(JsonValueKind::Boolean),
            position: 3
        }
    );
    assert_parse_error_matches!(
        "tr",
        JsonParseError::UnexpectedEos {
            kind: Some(JsonValueKind::Boolean),
            position: 2
        }
    );

    Ok(())
}

#[test]
fn parse_numbers() -> Result<(), JsonParseError> {
    // Integers.
    for text in ["0", "-12"] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Integer);
        assert_eq!(value.as_raw_str(), text);
        assert_eq!(value.position(), 0);
    }

    // Floats.
    for text in ["12.3", "12.3e4", "12.3e-4", "-0.3e+4", "12E034"] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Float);
        assert_eq!(value.as_raw_str(), text);
        assert_eq!(value.position(), 0);
    }

    // Malformed integers.
    for (text, position) in [("--1", 1)] {
        let e = assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::Integer),
                ..
            }
        );
        assert_eq!(e.position(), position);
    }

    // Malformed floats.
    for (text, position) in [("1..2", 2), ("1ee2", 2), ("1e+-3", 3)] {
        let e = assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::Float),
                ..
            }
        );
        assert_eq!(e.position(), position);
    }

    // Malformed values.
    for text in ["e123", "+2", ".123"] {
        assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedValueChar {
                kind: None,
                position: 0
            }
        );
    }

    // Unexpected trailing char.
    for (text, position) in [("123.4.5", 5), ("0123", 1), ("00", 1)] {
        let e = assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedTrailingChar {
                kind: JsonValueKind::Float | JsonValueKind::Integer,
                ..
            }
        );
        assert_eq!(e.position(), position);
    }

    // Unexpected EOS.
    for text in ["123.", "-", "123e", "123e-"] {
        assert_parse_error_matches!(text, JsonParseError::UnexpectedEos { .. });
    }

    Ok(())
}

#[test]
fn parse_strings() -> Result<(), JsonParseError> {
    // Non-escaped strings.
    for text in [r#" "" "#, r#" "abc" "#] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::String);
        assert_eq!(value.as_raw_str(), text.trim());
        assert_eq!(value.position(), 1);
        assert!(matches!(
            value.to_unquoted_string_str(),
            Ok(Cow::Borrowed(_))
        ));
    }

    // Escaped strings.
    for (text, unescaped) in [
        (r#" "ab\tc" "#, "ab\tc"),
        (r#" "\n\\a\r\nb\b\"\fc" "#, "\n\\a\r\nb\u{8}\"\u{c}c"),
        (r#" "ab\uF20ac" "#, "ab\u{f20a}c"),
    ] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::String);
        assert_eq!(value.as_raw_str(), text.trim());
        assert_eq!(value.position(), 1);
        assert!(matches!(value.to_unquoted_string_str(), Ok(Cow::Owned(_))));
        assert_eq!(value.to_unquoted_string_str().expect("ok"), unescaped);
    }

    // Malformed strings.
    for (text, error_position) in [(r#" "ab\xc" "#, 5), (r#" "ab\uXyz0c" "#, 6)] {
        let e = assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::String),
                ..
            }
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
        assert_parse_error_matches!(text, JsonParseError::UnexpectedEos { .. });
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
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Array);
        assert_eq!(value.as_raw_str(), text);
        assert_eq!(value.position(), 0);
    }

    // Malformed arrays.
    for (text, position) in [("[,]", 1), ("[1,2,]", 5)] {
        let e = assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::Array),
                ..
            }
        );
        assert_eq!(e.position(), position);
    }

    // Unmatched ']'.
    assert_parse_error_matches!(
        "]",
        JsonParseError::UnexpectedValueChar {
            kind: None,
            position: 0
        }
    );

    assert_parse_error_matches!(
        "[1,2]]",
        JsonParseError::UnexpectedTrailingChar {
            kind: JsonValueKind::Array,
            position: 5
        }
    );

    assert_parse_error_matches!(
        r#"{"foo":[]]}"#,
        JsonParseError::UnexpectedValueChar {
            kind: Some(JsonValueKind::Object),
            position: 9,
        }
    );

    // Unexpected EOS.
    for text in ["[", "[1,2", "[1,2,"] {
        assert_parse_error_matches!(text, JsonParseError::UnexpectedEos { .. });
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
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Object);
        assert_eq!(value.as_raw_str(), text);
        assert_eq!(value.position(), 0);
    }

    // Malformed objects.
    for (text, position) in [
        ("{,}", 1),
        ("{:}", 1),
        (r#"{"foo","bar"}"#, 6),
        (r#"{"foo":"bar",}"#, 13),
    ] {
        let e = assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::Object),
                ..
            }
        );
        assert_eq!(e.position(), position);
    }

    // Unmatched '}'.
    assert_parse_error_matches!(
        "}",
        JsonParseError::UnexpectedValueChar {
            kind: None,
            position: 0
        }
    );

    assert_parse_error_matches!(
        r#"{"1":2}}"#,
        JsonParseError::UnexpectedTrailingChar {
            kind: JsonValueKind::Object,
            position: 7
        }
    );

    assert_parse_error_matches!(
        "[{}}]",
        JsonParseError::UnexpectedValueChar {
            kind: Some(JsonValueKind::Array),
            position: 3
        }
    );

    // Unexpected EOS.
    for text in ["{", r#"{"1" "#, r#"{"1": "#, r#"{"1": 2"#] {
        assert_parse_error_matches!(text, JsonParseError::UnexpectedEos { .. });
    }

    Ok(())
}

#[test]
fn error_context() {
    let text = r#"
{
  "foo": "bar"
  "ba"
}
"#;
    let e = assert_parse_error_matches!(text, JsonParseError::UnexpectedValueChar { .. });
    assert_eq!(e.get_line(text), Some(r#"  "ba""#));
    assert_eq!(
        e.get_line_and_column_numbers(text)
            .map(|(l, c)| (l.get(), c.get())),
        Some((4, 3))
    );
}

#[test]
fn to_fixed_object() -> Result<(), JsonParseError> {
    struct Person {
        name: String,
        age: u32,
    }

    impl<'text> FromRawJsonValue<'text> for Person {
        fn from_raw_json_value(value: RawJsonValue<'text, '_>) -> Result<Self, JsonParseError> {
            let ([name, age], []) = value.to_fixed_object(["name", "age"], [])?;
            Ok(Person {
                name: name.try_to()?,
                age: age.try_to()?,
            })
        }
    }

    let person: Json<Person> = r#"{"name":"Alice","age":30}"#.parse()?;
    assert_eq!(person.0.name, "Alice");
    assert_eq!(person.0.age, 30);

    Ok(())
}

#[test]
fn parse_std_types() {
    assert_eq!("-1".parse().ok(), Some(Json(-1i8)));
    assert_eq!("\"a\"".parse().ok(), Some(Json("a".to_owned())));
    assert_eq!("123".parse().ok(), Some(Json(123u32)));
    assert_eq!("3.14".parse().ok(), Some(Json(3.14f64)));
    assert_eq!("true".parse().ok(), Some(Json(true)));
    assert_eq!("false".parse().ok(), Some(Json(false)));
    assert_eq!("null".parse::<Json<Option<bool>>>().ok(), Some(Json(None)));
    assert_eq!("true".parse().ok(), Some(Json(Some(true))));
    assert_eq!("[]".parse().ok(), Some(Json(())));
    assert_eq!("[1,true,0.2]".parse().ok(), Some(Json((1, true, 0.2))));
    assert_eq!("[1,2,3]".parse().ok(), Some(Json(vec![1, 2, 3])));
    assert_eq!("[[1],[2],[3]]".parse().ok(), Some(Json([[1], [2], [3]])));
    assert_eq!(
        r#"{"1":1,"2":null,"3":3}"#.parse().ok(),
        Some(Json(
            [(1, Some(1)), (2, None), (3, Some(3))]
                .into_iter()
                .collect::<BTreeMap<_, _>>()
        ))
    );
}

#[test]
fn get_value_by_position() {
    let json = RawJson::parse(r#"{"1":1,"2":null,"3":3}"#).expect("ok");

    let value = json.get_value_by_position(2).expect("some");
    assert_eq!(value.kind(), JsonValueKind::String);
    assert_eq!(value.position(), 1);
    assert_eq!(value.as_raw_str(), r#""1""#);

    let value = json.get_value_by_position(13).expect("some");
    assert_eq!(value.kind(), JsonValueKind::Null);
    assert_eq!(value.position(), 11);
}

#[test]
fn value_parent() {
    let text = r#"{"1":1,"2":[null],"3":3}"#;
    let json = RawJson::parse(text).expect("ok");
    let value = json.get_value_by_position(13).expect("some");
    assert_eq!(value.as_raw_str(), "null");

    let parent = value.parent().expect("parent");
    assert_eq!(parent.as_raw_str(), "[null]");

    let grand_parent = parent.parent().expect("parent");
    assert_eq!(grand_parent.as_raw_str(), text);
    assert_eq!(grand_parent.parent(), None);
}
