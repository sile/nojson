use nojson::{DisplayJson, Json, json};

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
