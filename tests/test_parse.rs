use std::borrow::Cow;

use nojson::{JsonParseError, JsonValueKind, RawJson};

#[test]
fn parse_empty_text() {
    assert!(matches!(
        RawJson::parse(""),
        Err(JsonParseError::UnexpectedEos {
            kind: None,
            position: 0
        })
    ));
    assert!(matches!(
        RawJson::parse("    "),
        Err(JsonParseError::UnexpectedEos {
            kind: None,
            position: 4
        })
    ));
}

#[test]
fn parse_nulls() -> Result<(), JsonParseError> {
    let json = RawJson::parse(" null ")?;
    let value = json.value();
    assert_eq!(value.kind(), JsonValueKind::Null);
    assert_eq!(value.text(), "null");
    assert_eq!(value.position(), 1);

    assert!(matches!(
        RawJson::parse("nuL"),
        Err(JsonParseError::UnexpectedValueChar {
            kind: Some(JsonValueKind::Null),
            position: 2
        })
    ));
    assert!(matches!(
        RawJson::parse("nul"),
        Err(JsonParseError::UnexpectedEos {
            kind: Some(JsonValueKind::Null),
            position: 3
        })
    ));
    assert!(matches!(
        RawJson::parse("nulla"),
        Err(JsonParseError::UnexpectedTrailingChar {
            kind: JsonValueKind::Null,
            position: 4
        })
    ));

    Ok(())
}

#[test]
fn parse_bools() -> Result<(), JsonParseError> {
    let json = RawJson::parse("true")?;
    let value = json.value();
    assert_eq!(value.kind(), JsonValueKind::Bool);
    assert_eq!(value.text(), "true");
    assert_eq!(value.position(), 0);

    let json = RawJson::parse(" false ")?;
    let value = json.value();
    assert_eq!(value.kind(), JsonValueKind::Bool);
    assert_eq!(value.text(), "false");
    assert_eq!(value.position(), 1);

    assert!(matches!(
        RawJson::parse("false true"),
        Err(JsonParseError::UnexpectedTrailingChar {
            kind: JsonValueKind::Bool,
            position: 6
        })
    ));
    assert!(matches!(
        RawJson::parse("fale"),
        Err(JsonParseError::UnexpectedValueChar {
            kind: Some(JsonValueKind::Bool),
            position: 3
        })
    ));
    assert!(matches!(
        RawJson::parse("tr"),
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
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Integer);
        assert_eq!(value.text(), text);
        assert_eq!(value.position(), 0);
    }

    // Floats.
    for text in ["12.3", "12.3e4", "12.3e-4", "-0.3e+4", "12E034"] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Float);
        assert_eq!(value.text(), text);
        assert_eq!(value.position(), 0);
    }

    // Malformed integers.
    for (text, position) in [("--1", 1)] {
        let e = RawJson::parse(text).expect_err("error");
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
        let e = RawJson::parse(text).expect_err("error");
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
                RawJson::parse(text),
                Err(JsonParseError::UnexpectedValueChar {
                    kind: None,
                    position: 0
                })
            ),
            "text={text}, error={:?}",
            RawJson::parse(text)
        );
    }

    // Unexpected trailing char.
    for (text, position) in [("123.4.5", 5), ("0123", 1), ("00", 1)] {
        let e = RawJson::parse(text).expect_err("error");
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
                RawJson::parse(text),
                Err(JsonParseError::UnexpectedEos { .. })
            ),
            "text={text}, error={:?}",
            RawJson::parse(text)
        );
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
        assert_eq!(value.text(), text.trim());
        assert_eq!(value.position(), 1);
        assert!(matches!(value.to_unquoted_str(), Cow::Borrowed(_)));
    }

    // Escaped strings.
    for text in [
        r#" "ab\tc" "#,
        r#" "\n\\a\r\nb\b\"\fc" "#,
        r#" "ab\uF20ac" "#,
    ] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::String);
        assert_eq!(value.text(), text.trim());
        assert_eq!(value.position(), 1);
        assert!(matches!(value.to_unquoted_str(), Cow::Owned(_)));
    }

    // Malformed strings.
    for (text, error_position) in [(r#" "ab\xc" "#, 5), (r#" "ab\uXyz0c" "#, 6)] {
        let e = RawJson::parse(text).expect_err("error");
        assert!(
            matches!(
                e,
                JsonParseError::UnexpectedValueChar {
                    kind: Some(JsonValueKind::String),
                    ..
                }
            ),
            "text={text}, error={:?}",
            RawJson::parse(text)
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
                RawJson::parse(text),
                Err(JsonParseError::UnexpectedEos { .. })
            ),
            "text={text}, error={:?}",
            RawJson::parse(text)
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
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::Array);
        assert_eq!(value.text(), text);
        assert_eq!(value.position(), 0);
    }

    // Malformed arrays.
    for (text, position) in [("[,]", 1), ("[1,2,]", 5)] {
        let e = RawJson::parse(text).expect_err("error");
        assert!(
            matches!(
                e,
                JsonParseError::UnexpectedValueChar {
                    kind: Some(JsonValueKind::Array),
                    ..
                }
            ),
            "text={text}, error={:?}",
            RawJson::parse(text)
        );
        assert_eq!(e.position(), position);
    }

    // Unmatched ']'.
    let text = "]";
    let e = RawJson::parse(text).expect_err("error");
    assert!(
        matches!(e, JsonParseError::UnexpectedValueChar { kind: None, .. }),
        "text={text}, error={e:?}",
    );
    assert_eq!(e.position(), 0);

    let text = "[1,2]]";
    let e = RawJson::parse(text).expect_err("error");
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
    let e = RawJson::parse(text).expect_err("error");
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
                RawJson::parse(text),
                Err(JsonParseError::UnexpectedEos { .. })
            ),
            "text={text}, error={:?}",
            RawJson::parse(text)
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
        let json = RawJson::parse(text)?;
        let value = json.value();
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
        let e = RawJson::parse(text).expect_err("error");
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
    let e = RawJson::parse(text).expect_err("error");
    assert!(
        matches!(e, JsonParseError::UnexpectedValueChar { kind: None, .. }),
        "text={text}, error={e:?}",
    );
    assert_eq!(e.position(), 0);

    let text = r#"{"1":2}}"#;
    let e = RawJson::parse(text).expect_err("error");
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
    let e = RawJson::parse(text).expect_err("error");
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
                RawJson::parse(text),
                Err(JsonParseError::UnexpectedEos { .. })
            ),
            "text={text}, error={:?}",
            RawJson::parse(text)
        );
    }

    Ok(())
}
