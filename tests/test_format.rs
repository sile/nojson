use std::{borrow::Cow, collections::BTreeMap};

use nojson::{DisplayJson, Json, json};

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
