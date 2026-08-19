use std::{borrow::Cow, collections::BTreeMap};

use nojson::{Json, JsonParseError, JsonValueKind, RawJson, RawJsonValue};

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
    {
        let (text, position) = ("--1", 1);
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
    for (text, unescaped) in [
        (r#" "" "#, ""),
        (r#" "abc" "#, "abc"),
        (r#" "あa" "#, "あa"),
        (r#" "日本語x" "#, "日本語x"),
    ] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::String);
        assert_eq!(value.as_raw_str(), text.trim());
        assert_eq!(value.position(), 1);
        assert!(matches!(
            value.to_unquoted_string_str(),
            Ok(Cow::Borrowed(_))
        ));
        assert_eq!(value.to_unquoted_string_str()?, unescaped);
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
        assert_eq!(value.to_unquoted_string_str()?, unescaped);
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
fn parse_surrogate_pairs() -> Result<(), JsonParseError> {
    // RFC 8259 §7: BMP-external code points may be written as a UTF-16
    // surrogate pair `\uXXXX\uXXXX`.
    for (text, unescaped) in [
        (r#" "\uD834\uDD1E" "#, "\u{1D11E}"),  // musical G clef
        (r#" "\uD83D\uDE00" "#, "\u{1F600}"),  // grinning face
        (r#" "\uD800\uDC00" "#, "\u{10000}"),  // lower boundary
        (r#" "\uDBFF\uDFFF" "#, "\u{10FFFF}"), // upper boundary
        (r#" "a\uD834\uDD1Eb" "#, "a\u{1D11E}b"),
        (r#" "\uD834\uDD1E\uD83D\uDE00" "#, "\u{1D11E}\u{1F600}"),
        (r#" "\uD83D\ude00" "#, "\u{1F600}"), // mixed case hex
    ] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.kind(), JsonValueKind::String);
        assert_eq!(value.as_raw_str(), text.trim());
        assert!(matches!(value.to_unquoted_string_str(), Ok(Cow::Owned(_))));
        assert_eq!(value.to_unquoted_string_str()?, unescaped);
    }

    // Non-regression: single `\uXXXX` for non-surrogate BMP code points,
    // including the boundaries just outside the surrogate range.
    for (text, unescaped) in [
        (r#" "\u00E9" "#, "\u{00E9}"), // e-acute
        (r#" "\uD7FF" "#, "\u{D7FF}"), // just below the surrogate range
        (r#" "\uE000" "#, "\u{E000}"), // just above the surrogate range
        (r#" "\uFFFD" "#, "\u{FFFD}"), // REPLACEMENT CHARACTER
    ] {
        let json = RawJson::parse(text)?;
        let value = json.value();
        assert_eq!(value.to_unquoted_string_str()?, unescaped);
    }

    // Malformed surrogate sequences that produce `UnexpectedValueChar`. The
    // `position` points at whatever character actually broke the pair rule
    // (missing `\u`, out-of-range low, or a lone low), which is more useful
    // than always pointing at the first hex digit of the offending escape.
    for (text, error_position) in [
        (r#" "\uD834" "#, 8),        // lone high surrogate → closing quote
        (r#" "\uDD1E" "#, 4),        // lone low surrogate → first hex digit
        (r#" "\uD834x" "#, 8),       // high followed by a non-\u char → `x`
        (r#" "\uD834\n" "#, 8),      // high followed by an escape that isn't \u → `\`
        (r#" "\uD834\u0041" "#, 10), // high followed by non-surrogate BMP → second hex
        (r#" "\uD834\uD7FF" "#, 10), // low value just below the low-surrogate range
        (r#" "\uD834\uE000" "#, 10), // low value just above the low-surrogate range
        (r#" "\uD834\uD800" "#, 10), // "low" value is actually another high
    ] {
        let e = assert_parse_error_matches!(
            text,
            JsonParseError::UnexpectedValueChar {
                kind: Some(JsonValueKind::String),
                ..
            }
        );
        assert_eq!(e.position(), error_position, "text={text}");
    }

    // Truncated surrogate sequences that produce `UnexpectedEos`.
    for text in [
        r#" "\uD834"#,   // truncated right after the high surrogate
        r#" "\uD834\u"#, // truncated right after the second \u
        r#" "\uD834\u0"#,
        r#" "\uD834\u00"#,
        r#" "\uD834\u000"#,
    ] {
        assert_parse_error_matches!(text, JsonParseError::UnexpectedEos { .. });
    }

    Ok(())
}

#[test]
fn parse_surrogate_pair_in_object_key() -> Result<(), JsonParseError> {
    // A surrogate pair in an object key must be composed by `unquote`
    // before it is compared against the caller-supplied name in
    // `find_member_by_name`.
    let text = r#"{"\uD834\uDD1E": 1}"#;
    let json = RawJson::parse(text)?;
    let value = json.value().to_member("\u{1D11E}")?.required()?;
    let n: u32 = value.try_into()?;
    assert_eq!(n, 1);
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

    // Test for unexpected EOS case
    let text_eof = r#"[
"foo"#;
    let e = assert_parse_error_matches!(text_eof, JsonParseError::UnexpectedEos { .. });
    assert_eq!(e.get_line(text_eof), Some(r#""foo"#));
    assert_eq!(
        e.get_line_and_column_numbers(text_eof)
            .map(|(l, c)| (l.get(), c.get())),
        Some((2, 5))
    );
}

#[test]
fn to_member_required() -> Result<(), JsonParseError> {
    struct Person {
        name: String,
        age: u32,
    }

    impl<'text, 'raw> TryFrom<RawJsonValue<'text, 'raw>> for Person {
        type Error = JsonParseError;

        fn try_from(value: RawJsonValue<'text, 'raw>) -> Result<Self, Self::Error> {
            let name = value.to_member("name")?.required()?;
            let age = value.to_member("age")?.required()?;
            Ok(Person {
                name: name.try_into()?,
                age: age.try_into()?,
            })
        }
    }

    let person: Json<Person> = r#"{"name":"Alice","age":30}"#.parse()?;
    assert_eq!(person.0.name, "Alice");
    assert_eq!(person.0.age, 30);

    Ok(())
}

#[test]
fn to_member_optional() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"name":"Alice","age":30}"#)?;
    let value = json.value();

    let name: String = value
        .to_member("name")?
        .optional()
        .expect("some")
        .try_into()?;
    assert_eq!(name, "Alice");

    let city = value.to_member("city")?.optional();
    assert_eq!(city, None);

    Ok(())
}

#[test]
fn required_member_missing() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"name":"Alice"}"#)?;
    let e = json
        .value()
        .to_member("age")?
        .required()
        .expect_err("required member should be missing");
    assert!(matches!(
        e,
        JsonParseError::InvalidValue {
            kind: JsonValueKind::Object,
            position: 0,
            ..
        }
    ));
    assert!(
        e.to_string().contains("required member 'age' is missing"),
        "unexpected error: {e}"
    );
    Ok(())
}

#[test]
fn member_access_requires_object() -> Result<(), JsonParseError> {
    let json = RawJson::parse("null")?;

    let e = json
        .value()
        .to_member("x")
        .expect_err("non-object should fail");
    assert!(matches!(
        e,
        JsonParseError::InvalidValue {
            kind: JsonValueKind::Null,
            position: 0,
            ..
        }
    ));
    assert!(
        e.to_string().contains("expected Object, but found Null"),
        "unexpected error: {e}"
    );

    Ok(())
}

#[test]
fn to_member_optional_try_from() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"n":42}"#)?;
    let value = json.value();

    let n1: Option<u64> = value.to_member("n")?.map(u64::try_from)?;
    assert_eq!(n1, Some(42));

    let n2: Option<u64> = value
        .to_member("n")?
        .optional()
        .map(u64::try_from)
        .transpose()?;
    assert_eq!(n2, Some(42));

    let missing: Option<u64> = value
        .to_member("missing")?
        .optional()
        .map(u64::try_from)
        .transpose()?;
    assert_eq!(missing, None);

    Ok(())
}

#[test]
fn to_path_member_success_required() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"a":{"b":{"c":42}}}"#)?;
    let value = json.value();

    let c: u64 = value
        .to_path_member(&["a", "b", "c"])?
        .required()?
        .try_into()?;
    assert_eq!(c, 42);

    Ok(())
}

#[test]
fn to_path_member_final_missing_optional() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"a":{"b":{}}}"#)?;
    let value = json.value();

    let missing = value.to_path_member(&["a", "b", "c"])?.optional();
    assert_eq!(missing, None);

    Ok(())
}

#[test]
fn to_path_member_empty_path_error() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"a":1}"#)?;
    let e = json
        .value()
        .to_path_member(&[])
        .expect_err("empty path should fail");
    assert!(matches!(
        e,
        JsonParseError::InvalidValue {
            kind: JsonValueKind::Object,
            position: 0,
            ..
        }
    ));
    assert!(
        e.to_string().contains("path must not be empty"),
        "unexpected error: {e}"
    );

    Ok(())
}

#[test]
fn to_path_member_intermediate_missing_error() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"a":{}}"#)?;
    let e = json
        .value()
        .to_path_member(&["a", "b", "c"])
        .expect_err("intermediate missing should fail");
    assert!(matches!(
        e,
        JsonParseError::InvalidValue {
            kind: JsonValueKind::Object,
            ..
        }
    ));
    assert!(
        e.to_string().contains("required member 'b' is missing"),
        "unexpected error: {e}"
    );

    Ok(())
}

#[test]
fn to_path_member_intermediate_not_object_error() -> Result<(), JsonParseError> {
    let json = RawJson::parse(r#"{"a":1}"#)?;
    let e = json
        .value()
        .to_path_member(&["a", "b"])
        .expect_err("non-object intermediate should fail");
    assert!(matches!(
        e,
        JsonParseError::InvalidValue {
            kind: JsonValueKind::Integer,
            ..
        }
    ));
    assert!(
        e.to_string().contains("expected Object, but found Integer"),
        "unexpected error: {e}"
    );

    Ok(())
}

#[test]
fn to_path_member_root_not_object_error() -> Result<(), JsonParseError> {
    let json = RawJson::parse("null")?;
    let e = json
        .value()
        .to_path_member(&["x"])
        .expect_err("root non-object should fail");
    assert!(matches!(
        e,
        JsonParseError::InvalidValue {
            kind: JsonValueKind::Null,
            position: 0,
            ..
        }
    ));
    assert!(
        e.to_string().contains("expected Object, but found Null"),
        "unexpected error: {e}"
    );

    Ok(())
}

#[test]
fn parse_std_types() {
    assert_eq!("-1".parse().ok(), Some(Json(-1i8)));
    assert_eq!("\"a\"".parse().ok(), Some(Json("a".to_owned())));
    assert_eq!("123".parse().ok(), Some(Json(123u32)));
    assert_eq!("3.45".parse().ok(), Some(Json(3.45f64)));
    assert_eq!("true".parse().ok(), Some(Json(true)));
    assert_eq!("false".parse().ok(), Some(Json(false)));
    assert_eq!("null".parse().ok(), Some(Json(())));
    assert_eq!("null".parse::<Json<Option<bool>>>().ok(), Some(Json(None)));
    assert_eq!("true".parse().ok(), Some(Json(Some(true))));
    assert_eq!("[]".parse().ok(), Some(Json::<[usize; 0]>([])));
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

// --- Nesting-depth limit ---------------------------------------------
//
// The parser rejects inputs whose nesting would step past
// `MAX_NESTING_DEPTH`. `nested_arrays(n)` / `nested_objects(n)` build
// inputs of the requested depth so tests can pin the boundary.

fn nested_arrays(depth: usize) -> String {
    let mut s = String::with_capacity(depth * 2);
    for _ in 0..depth {
        s.push('[');
    }
    for _ in 0..depth {
        s.push(']');
    }
    s
}

fn nested_objects(depth: usize) -> String {
    // Single-key chain: `{"k":{"k":...null...}}`.
    let mut s = String::with_capacity(depth * 6 + 4);
    for _ in 0..depth {
        s.push_str("{\"k\":");
    }
    s.push_str("null");
    for _ in 0..depth {
        s.push('}');
    }
    s
}

#[test]
fn parse_nesting_at_limit_succeeds() -> Result<(), JsonParseError> {
    let arr = nested_arrays(nojson::MAX_NESTING_DEPTH);
    let value = RawJson::parse(&arr)?;
    assert_eq!(value.value().kind(), JsonValueKind::Array);

    let obj = nested_objects(nojson::MAX_NESTING_DEPTH);
    let value = RawJson::parse(&obj)?;
    assert_eq!(value.value().kind(), JsonValueKind::Object);
    Ok(())
}

fn assert_nesting_too_deep(e: &JsonParseError, expected_kind: JsonValueKind, expected_pos: usize) {
    // The depth-limit rejection uses `InvalidValue` with a string error
    // carrying "nesting depth exceeded".
    assert!(
        matches!(e, JsonParseError::InvalidValue { .. }),
        "expected InvalidValue, got {e:?}"
    );
    assert_eq!(e.kind(), Some(expected_kind), "error: {e:?}");
    assert_eq!(e.position(), expected_pos, "error: {e:?}");
    let msg = e.to_string();
    assert!(
        msg.contains("nesting depth exceeded"),
        "message does not mention nesting depth: {msg}"
    );
    // Locks the message value in sync with `MAX_NESTING_DEPTH` — catches a
    // future refactor that hardcodes a different number in the message.
    let expected_value = format!("({})", nojson::MAX_NESTING_DEPTH);
    assert!(
        msg.contains(&expected_value),
        "message does not mention MAX_NESTING_DEPTH value {expected_value}: {msg}"
    );
}

#[test]
fn parse_nesting_over_limit_errors() {
    let arr = nested_arrays(nojson::MAX_NESTING_DEPTH + 1);
    // The offending '[' sits at index MAX_NESTING_DEPTH (0-indexed).
    let e = RawJson::parse(&arr).expect_err("over-limit array must fail");
    assert_nesting_too_deep(&e, JsonValueKind::Array, nojson::MAX_NESTING_DEPTH);

    let obj = nested_objects(nojson::MAX_NESTING_DEPTH + 1);
    // Each object level in `nested_objects` is `{"k":` (5 bytes), so the
    // outermost over-limit '{' sits at byte position MAX_NESTING_DEPTH * 5.
    let e = RawJson::parse(&obj).expect_err("over-limit object must fail");
    assert_nesting_too_deep(&e, JsonValueKind::Object, nojson::MAX_NESTING_DEPTH * 5);
}

#[test]
fn parse_nesting_over_limit_jsonc_with_comments_errors() {
    // JSONC-mode over-limit: each level is `/*x*/[` (6 bytes) so the offending
    // 129th `[` sits after 128 full levels and one more comment prefix,
    // i.e. at MAX_NESTING_DEPTH * 6 + 5. Pins that comment-skip does not
    // desync the position reported by the depth check.
    let mut text = String::new();
    for _ in 0..(nojson::MAX_NESTING_DEPTH + 1) {
        text.push_str("/*x*/[");
    }
    for _ in 0..(nojson::MAX_NESTING_DEPTH + 1) {
        text.push(']');
    }
    let e = RawJson::parse_jsonc(&text).expect_err("over-limit JSONC must fail");
    assert_nesting_too_deep(&e, JsonValueKind::Array, nojson::MAX_NESTING_DEPTH * 6 + 5);
}

#[test]
fn parse_nesting_over_limit_jsonc_object_with_comments_errors() {
    // Object variant: each level is `{"k":/*x*/` (10 bytes). `parse_object_inner`
    // invokes `skip_whitespaces_and_comments` at four points per level (before
    // the key, after `:`, after `,`, before the closing `}`), so an
    // Array-only JSONC test does not cover the object comment-skip path.
    let mut text = String::new();
    for _ in 0..(nojson::MAX_NESTING_DEPTH + 1) {
        text.push_str("{\"k\":/*x*/");
    }
    text.push_str("null");
    for _ in 0..(nojson::MAX_NESTING_DEPTH + 1) {
        text.push('}');
    }
    let e = RawJson::parse_jsonc(&text).expect_err("over-limit JSONC object must fail");
    assert_nesting_too_deep(&e, JsonValueKind::Object, nojson::MAX_NESTING_DEPTH * 10);
}

fn assert_all_entry_points_reject(text: &str, expected_kind: JsonValueKind, expected_pos: usize) {
    let e = RawJson::parse(text).expect_err("RawJson::parse should reject");
    assert_nesting_too_deep(&e, expected_kind, expected_pos);

    let e = RawJson::parse_jsonc(text).expect_err("RawJson::parse_jsonc should reject");
    assert_nesting_too_deep(&e, expected_kind, expected_pos);

    let e = nojson::RawJsonOwned::parse(text).expect_err("RawJsonOwned::parse should reject");
    assert_nesting_too_deep(&e, expected_kind, expected_pos);

    let e = nojson::RawJsonOwned::parse_jsonc(text)
        .expect_err("RawJsonOwned::parse_jsonc should reject");
    assert_nesting_too_deep(&e, expected_kind, expected_pos);

    let e = text
        .parse::<nojson::RawJsonOwned>()
        .expect_err("RawJsonOwned::from_str should reject");
    assert_nesting_too_deep(&e, expected_kind, expected_pos);

    // `Json::<T>::from_str` rejects at parse time before `TryFrom` runs, so
    // the placeholder `T` never matters here.
    let e = text
        .parse::<Json<Vec<()>>>()
        .expect_err("Json::<T>::from_str should reject");
    assert_nesting_too_deep(&e, expected_kind, expected_pos);
}

#[test]
fn parse_nesting_over_limit_all_entry_points() {
    // Every public entry point that reaches the parser must report the same
    // depth-limit rejection for over-limit input, for both Array and Object
    // roots (kind symmetry — one wrapper's decrement bug would be masked if
    // only Array were checked).
    let arr = nested_arrays(nojson::MAX_NESTING_DEPTH + 1);
    assert_all_entry_points_reject(&arr, JsonValueKind::Array, nojson::MAX_NESTING_DEPTH);

    // Each object level in `nested_objects` is `{"k":` (5 bytes), so the
    // outermost over-limit '{' sits at MAX_NESTING_DEPTH * 5.
    let obj = nested_objects(nojson::MAX_NESTING_DEPTH + 1);
    assert_all_entry_points_reject(&obj, JsonValueKind::Object, nojson::MAX_NESTING_DEPTH * 5);
}

#[test]
fn parse_nesting_siblings_at_limit_succeed() -> Result<(), JsonParseError> {
    // Depth is a shared counter: closing a container must free a slot so
    // sibling containers can each reach the limit independently. If
    // `parse_array` / `parse_object` forgot to decrement after finishing,
    // the second sibling here would trip the depth check even though the
    // total depth at any point is only `MAX_NESTING_DEPTH`.
    let inner = nested_arrays(nojson::MAX_NESTING_DEPTH - 1);
    let text = format!("[{inner},{inner}]");
    RawJson::parse(&text)?;
    Ok(())
}

#[test]
fn parse_nesting_at_limit_jsonc_with_comments_succeed() -> Result<(), JsonParseError> {
    // JSONC's `skip_whitespaces_and_comments` runs at each container open, so
    // interleaving comments with 128-deep nesting exercises depth counting
    // through the comment-skip path (a plain-JSON at-limit test does not).
    let mut text = String::new();
    for _ in 0..nojson::MAX_NESTING_DEPTH {
        text.push_str("[/*x*/");
    }
    for _ in 0..nojson::MAX_NESTING_DEPTH {
        text.push(']');
    }
    let (_, comments) = RawJson::parse_jsonc(&text)?;
    assert_eq!(comments.len(), nojson::MAX_NESTING_DEPTH);
    Ok(())
}

#[test]
fn parse_nesting_object_siblings_at_limit_succeed() -> Result<(), JsonParseError> {
    // Object version of the sibling test. `parse_array` and `parse_object`
    // are independent wrappers, so a decrement bug can slip into just one of
    // them without the Array-only sibling test noticing.
    let inner = nested_objects(nojson::MAX_NESTING_DEPTH - 1);
    let text = format!(r#"{{"a":{inner},"b":{inner}}}"#);
    RawJson::parse(&text)?;
    Ok(())
}

// The three `RawJsonOwned::{object, json, array}` builders re-parse their own
// formatter output, so a formatter that emits > MAX_NESTING_DEPTH nesting
// makes them panic. These `#[should_panic]` tests pin that panic contract so
// a future refactor cannot silently drop it. The `expected` substring
// `"nesting depth exceeded"` appears in the depth error's Debug output only,
// so panics from unrelated formatter bugs would not satisfy the assertion.

#[test]
#[should_panic(expected = "nesting depth exceeded")]
fn raw_json_owned_json_panics_over_depth() {
    // 129-deep `[...]` re-parsed by RawJsonOwned::parse trips the depth cap.
    let deep = nested_arrays(nojson::MAX_NESTING_DEPTH + 1);
    let _ = nojson::RawJsonOwned::json(|f| write!(f.inner_mut(), "{deep}"));
}

#[test]
#[should_panic(expected = "nesting depth exceeded")]
fn raw_json_owned_array_panics_over_depth() {
    // Outer array (depth 1) + a 128-deep inner array raw-written via
    // `f.inner_mut()` totals depth 129, past the cap.
    let inner = nested_arrays(nojson::MAX_NESTING_DEPTH);
    let _ = nojson::RawJsonOwned::array(|f| {
        f.element(nojson::json(|f| write!(f.inner_mut(), "{inner}")))
    });
}

#[test]
#[should_panic(expected = "nesting depth exceeded")]
fn raw_json_owned_object_panics_over_depth() {
    // Outer object (depth 1) + a 128-deep inner array value totals depth 129.
    let inner = nested_arrays(nojson::MAX_NESTING_DEPTH);
    let _ = nojson::RawJsonOwned::object(|f| {
        f.member("k", nojson::json(|f| write!(f.inner_mut(), "{inner}")))
    });
}

// Counterpart to the `should_panic` tests: MAX-deep formatter output must
// not panic when re-parsed. Guards against off-by-one refactors that would
// flip the depth check to `>` and break the builder's boundary behaviour.

#[test]
fn raw_json_owned_json_succeeds_at_max() {
    let deep = nested_arrays(nojson::MAX_NESTING_DEPTH);
    let _ = nojson::RawJsonOwned::json(|f| write!(f.inner_mut(), "{deep}"));
}

#[test]
fn raw_json_owned_array_succeeds_at_max() {
    // Outer array (depth 1) + a (MAX - 1)-deep inner totals exactly MAX.
    let inner = nested_arrays(nojson::MAX_NESTING_DEPTH - 1);
    let _ = nojson::RawJsonOwned::array(|f| {
        f.element(nojson::json(|f| write!(f.inner_mut(), "{inner}")))
    });
}

#[test]
fn raw_json_owned_object_succeeds_at_max() {
    // Outer object (depth 1) + a (MAX - 1)-deep inner totals exactly MAX.
    let inner = nested_arrays(nojson::MAX_NESTING_DEPTH - 1);
    let _ = nojson::RawJsonOwned::object(|f| {
        f.member("k", nojson::json(|f| write!(f.inner_mut(), "{inner}")))
    });
}

#[test]
fn parse_nesting_mixed_over_limit_errors() {
    // 128 objects opened, then a 129th container (an array) — a shared depth
    // counter rejects at the 129th, so this exercises depth counting across
    // Array / Object mixed nesting. If someone refactored to per-kind
    // counters, the array counter would still be at 0 and this input would
    // slip past the check.
    let mut text = String::new();
    for _ in 0..nojson::MAX_NESTING_DEPTH {
        text.push_str("{\"k\":");
    }
    text.push('[');
    text.push_str("null");
    text.push(']');
    for _ in 0..nojson::MAX_NESTING_DEPTH {
        text.push('}');
    }
    // `{"k":` is 5 bytes; the first `[` sits at MAX_NESTING_DEPTH * 5.
    let e = RawJson::parse(&text).expect_err("mixed over-limit nesting must fail");
    assert_nesting_too_deep(&e, JsonValueKind::Array, nojson::MAX_NESTING_DEPTH * 5);
}
