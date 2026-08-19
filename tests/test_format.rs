use std::{borrow::Cow, collections::BTreeMap, fmt::Write as _};

use nojson::{DisplayJson, Json, JsonFormatter, JsonParseError, json};

#[test]
fn float() {
    assert_eq!(json(|f| f.value(1.23f32)).to_string(), "1.23");
    assert_eq!(json(|f| f.value(1.23f64)).to_string(), "1.23");

    assert_eq!(json(|f| f.value(f32::NAN)).to_string(), "null");
    assert_eq!(json(|f| f.value(f64::NAN)).to_string(), "null");
    assert_eq!(json(|f| f.value(f32::INFINITY)).to_string(), "null");
    assert_eq!(json(|f| f.value(f64::INFINITY)).to_string(), "null");
}

#[test]
fn string() {
    assert_eq!(
        json(|f| f.value(Cow::Borrowed("foo"))).to_string(),
        "\"foo\""
    );
}

#[test]
fn array() {
    assert_eq!(Json([1, 2, 3]).to_string(), "[1,2,3]");
    assert_eq!(Json([Some(1), None, Some(3)]).to_string(), "[1,null,3]");

    assert_eq!(
        format!(
            "\n{}",
            json(|f| {
                f.set_indent_size(2);
                f.set_spacing(true);
                f.value([1, 2, 3])
            })
        ),
        r#"
[
  1,
  2,
  3
]"#
    );

    assert_eq!(
        format!(
            "\n{}",
            json(|f| {
                f.set_indent_size(2);
                f.set_spacing(true);
                f.value([vec![1], vec![2, 3]])
            })
        ),
        r#"
[
  [
    1
  ],
  [
    2,
    3
  ]
]"#
    );
    assert_eq!(
        format!(
            "\n{}",
            json(|f| {
                f.set_indent_size(2);
                f.set_spacing(true);
                f.value([
                    &vec![1] as &dyn DisplayJson,
                    &json(|f| {
                        f.set_indent_size(0);
                        f.value(vec![2, 3])
                    }),
                ])
            })
        ),
        r#"
[
  [
    1
  ],
  [2, 3]
]"#
    );
}

#[test]
fn object() {
    let object = [(1, None), (2, Some("foo")), (3, Some("ba\nr"))]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        Json(&object).to_string(),
        r#"{"1":null,"2":"foo","3":"ba\nr"}"#
    );
    assert_eq!(
        json(|f| {
            f.set_spacing(true);
            f.value(&object)
        })
        .to_string(),
        r#"{ "1": null, "2": "foo", "3": "ba\nr" }"#
    );
    assert_eq!(
        format!(
            "\n{}",
            json(|f| {
                f.set_indent_size(2);
                f.set_spacing(true);
                f.value(&object)
            })
        ),
        r#"
{
  "1": null,
  "2": "foo",
  "3": "ba\nr"
}"#
    );
    assert_eq!(
        format!(
            "\n{}",
            json(|f| {
                f.set_indent_size(2);
                f.set_spacing(true);
                f.value([&object])
            })
        ),
        r#"
[
  {
    "1": null,
    "2": "foo",
    "3": "ba\nr"
  }
]"#
    );
}

#[test]
fn raw_json() {
    let text = r#"[ {"user": {"name": "John", "age": 30}, "count": 42} ]"#;
    let json = nojson::RawJson::parse(text).expect("bug");
    assert_eq!(
        json.to_string(),
        r#"[{"user":{"name":"John","age":30},"count":42}]"#
    );
}

#[test]
fn raw_json_owned_object() -> Result<(), JsonParseError> {
    let raw = nojson::RawJsonOwned::object(|f| {
        f.member("name", "Alice")?;
        f.member("age", 30)
    });
    assert_eq!(raw.to_string(), r#"{"name":"Alice","age":30}"#);

    let name: String = raw.value().to_member("name")?.required()?.try_into()?;
    assert_eq!(name, "Alice");
    Ok(())
}

#[test]
fn raw_json_owned_json() -> Result<(), JsonParseError> {
    let raw = nojson::RawJsonOwned::json(|f| f.value([1, 2, 3]));
    assert_eq!(raw.to_string(), "[1,2,3]");

    let values: Vec<u8> = raw.value().try_into()?;
    assert_eq!(values, vec![1, 2, 3]);
    Ok(())
}

#[test]
fn raw_json_owned_array() -> Result<(), JsonParseError> {
    let raw = nojson::RawJsonOwned::array(|f| {
        f.element("Alice")?;
        f.element(30)
    });
    assert_eq!(raw.to_string(), r#"["Alice",30]"#);

    let mut values = raw.value().to_array()?;
    let name: String = values.next().expect("some").try_into()?;
    let age: u32 = values.next().expect("some").try_into()?;
    assert_eq!(name, "Alice");
    assert_eq!(age, 30);
    assert_eq!(values.next(), None);
    Ok(())
}

// Regression tests for indent-size / nesting-level combinations that used to
// hit `core::fmt`'s internal `u16` width limit and panic with "Formatting
// argument out of range".

#[test]
fn set_indent_size_max_does_not_panic_when_formatting() {
    // With `usize::MAX`, the loop-based `indent()` would emit `usize::MAX`
    // spaces (after `saturating_mul`), so route the output through a writer
    // that caps total bytes to keep the test fast and avoid OOM. Asserting
    // that the write errors out proves the loop actually ran and hit the
    // cap; if a future refactor silently clipped the indent size, the write
    // would succeed and this assertion would catch that regression.
    struct CappedWriter {
        written: usize,
        limit: usize,
    }
    impl core::fmt::Write for CappedWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.written = self.written.saturating_add(s.len());
            if self.written > self.limit {
                Err(core::fmt::Error)
            } else {
                Ok(())
            }
        }
    }
    let output = json(|f| {
        f.set_indent_size(usize::MAX);
        f.array(|f| f.element(1))
    });
    let mut w = CappedWriter {
        written: 0,
        limit: 4096,
    };
    assert!(write!(w, "{output}").is_err());
}

#[test]
fn deep_nesting_crossing_u16_width_does_not_panic() {
    // Cross the historical `u16::MAX = 65535` width panic threshold by
    // combining a modest `indent_size` with deep nesting:
    // `indent_size * level = 128 * 513 = 65664` at the deepest indent.
    // Discard the output so we don't pay for the ~34MB it would materialize
    // (~17MB for opening indents plus ~17MB for closing indents).
    struct DiscardingWriter;
    impl core::fmt::Write for DiscardingWriter {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    struct Nested(u32);
    impl DisplayJson for Nested {
        fn fmt(&self, f: &mut JsonFormatter<'_, '_>) -> core::fmt::Result {
            if self.0 == 0 {
                f.value(0)
            } else {
                f.array(|f| f.element(Nested(self.0 - 1)))
            }
        }
    }

    let output = json(|f| {
        f.set_indent_size(128);
        f.value(Nested(513))
    });
    write!(DiscardingWriter, "{output}").unwrap();
}
