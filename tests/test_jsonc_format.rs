use nojson::{JsoncFormatter, JsoncLineBreaks, JsoncTrailingCommas};

fn fmt(line_breaks: JsoncLineBreaks, indent: usize, commas: JsoncTrailingCommas) -> JsoncFormatter {
    JsoncFormatter {
        indent_size: indent,
        line_breaks,
        trailing_commas: commas,
    }
}

const PRESERVE: JsoncLineBreaks = JsoncLineBreaks::Preserve;
const ALWAYS: JsoncLineBreaks = JsoncLineBreaks::Always;
const TC_PRESERVE: JsoncTrailingCommas = JsoncTrailingCommas::Preserve;
const TC_ALWAYS: JsoncTrailingCommas = JsoncTrailingCommas::AlwaysMultiline;
const TC_NEVER: JsoncTrailingCommas = JsoncTrailingCommas::Never;

#[test]
fn basic_single_line() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(f.format(r#"[1, 2, 3]"#).unwrap(), "[1, 2, 3]");
    assert_eq!(f.format(r#"[ 1,2 ,3]"#).unwrap(), "[1, 2, 3]");
    assert_eq!(f.format(r#"{"a":1,"b":2}"#).unwrap(), r#"{"a": 1, "b": 2}"#);
    assert_eq!(f.format(r#"[]"#).unwrap(), "[]");
    assert_eq!(f.format(r#"{}"#).unwrap(), "{}");
    assert_eq!(f.format(r#"[ ]"#).unwrap(), "[]");
    assert_eq!(f.format(r#"42"#).unwrap(), "42");
    assert_eq!(f.format(r#""hi""#).unwrap(), r#""hi""#);
}

#[test]
fn single_line_comment_only() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(f.format(r#"[/* c */]"#).unwrap(), "[ /* c */ ]");
    assert_eq!(f.format(r#"{ /* c */ }"#).unwrap(), "{ /* c */ }");
}

#[test]
fn single_line_with_comments() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(f.format(r#"[/* a */1, 2]"#).unwrap(), "[ /* a */ 1, 2]");
    assert_eq!(f.format(r#"[1 /* a */, 2]"#).unwrap(), "[1 /* a */, 2]");
    assert_eq!(f.format(r#"[1, /* a */ 2]"#).unwrap(), "[1, /* a */ 2]");
    assert_eq!(f.format(r#"[1, 2 /* a */]"#).unwrap(), "[1, 2 /* a */]");
    assert_eq!(
        f.format(r#"{"a"/* k */:1}"#).unwrap(),
        r#"{"a" /* k */: 1}"#
    );
    assert_eq!(
        f.format(r#"{"a":/* v */1}"#).unwrap(),
        r#"{"a": /* v */ 1}"#
    );
}

#[test]
fn preserve_multiline() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(
        f.format("{\n  \"a\": 1,\n  \"b\": 2\n}").unwrap(),
        "{\n  \"a\": 1,\n  \"b\": 2\n}"
    );
    assert_eq!(f.format("[1,\n 2]").unwrap(), "[\n  1,\n  2\n]");
}

#[test]
fn always_multiline() {
    let f = fmt(ALWAYS, 2, TC_NEVER);
    assert_eq!(f.format("[1, 2]").unwrap(), "[\n  1,\n  2\n]");
    assert_eq!(f.format("[]").unwrap(), "[]");
    assert_eq!(f.format("{}").unwrap(), "{}");
}

#[test]
fn trailing_commas() {
    assert_eq!(
        fmt(PRESERVE, 2, TC_PRESERVE).format("[1, 2,]").unwrap(),
        "[1, 2,]"
    );
    assert_eq!(
        fmt(PRESERVE, 2, TC_ALWAYS).format("[1, 2,]").unwrap(),
        "[1, 2]"
    );
    assert_eq!(
        fmt(ALWAYS, 2, TC_ALWAYS).format("[1, 2]").unwrap(),
        "[\n  1,\n  2,\n]"
    );
    assert_eq!(
        fmt(ALWAYS, 2, TC_NEVER).format("[1, 2,]").unwrap(),
        "[\n  1,\n  2\n]"
    );
}

#[test]
fn invalid_input_errors() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert!(f.format("{not valid}").is_err());
    assert!(f.format("").is_err());
    // A document with only comments has no value.
    assert!(f.format("// only a comment").is_err());
    assert!(f.format("/* only a comment */").is_err());
    // An unterminated block comment is invalid.
    assert!(f.format("[1, /* unterminated").is_err());
    // A trailing comment after a valid value is accepted.
    assert!(f.format("[1] // done").is_ok());
    // Nesting deeper than MAX_NESTING_DEPTH is rejected.
    let depth = nojson::MAX_NESTING_DEPTH + 1;
    let deep = format!("{}1{}", "[".repeat(depth), "]".repeat(depth));
    assert!(f.format(&deep).is_err());
}

#[test]
fn unicode_strings_and_keys_preserved() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // Multi-byte scalar lexemes are re-emitted verbatim; byte spans stay on
    // UTF-8 boundaries even next to comments.
    assert_eq!(
        f.format(r#"["日本語", "😀🚀"]"#).unwrap(),
        r#"["日本語", "😀🚀"]"#
    );
    assert_eq!(
        f.format("[\n  \"日本語\" // c\n]").unwrap(),
        "[\n  \"日本語\" // c\n]"
    );
    assert_eq!(
        f.format("{\n  \"名前\" /* k */: 1\n}").unwrap(),
        "{\n  \"名前\" /* k */: 1\n}"
    );
    assert_eq!(
        f.format(r#"{"名前"/* k */:1}"#).unwrap(),
        r#"{"名前" /* k */: 1}"#
    );
    assert_eq!(
        f.format("[\n  \"😀\",\n  /* a\n  b */\n  1\n]").unwrap(),
        "[\n  \"😀\",\n  /* a\n  b */\n  1\n]"
    );
}

#[test]
fn preserve_single_line_siblings() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    let input = "{\n  \"left\": [\n    1,\n    2\n  ],\n  \"right\": [3, 4], \"nested\": {\"items\": [5, 6]}\n}";
    let out = f.format(input).unwrap();
    assert_eq!(
        out,
        "{\n  \"left\": [\n    1,\n    2\n  ],\n  \"right\": [3, 4],\n  \"nested\": {\"items\": [5, 6]}\n}"
    );
}

#[test]
fn empty_containers_stay_single_line() {
    // Always: empty [] / {} stay single-line even beside multi-line siblings.
    let f = fmt(ALWAYS, 2, TC_ALWAYS);
    assert_eq!(
        f.format("{\n  \"a\": [],\n  \"b\": [1, 2]\n}").unwrap(),
        "{\n  \"a\": [],\n  \"b\": [\n    1,\n    2,\n  ],\n}"
    );
    // Preserve: single-line containers stay single-line beside multi-line ones.
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(
        f.format("{\n  \"a\": [],\n  \"b\": [1,\n 2]\n}").unwrap(),
        "{\n  \"a\": [],\n  \"b\": [\n    1,\n    2\n  ]\n}"
    );
}

#[test]
fn line_comments_force_multiline() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // A line comment makes the container multiline in Preserve too.
    assert_eq!(
        f.format(
            r#"["a" // c
]"#
        )
        .unwrap(),
        "[\n  \"a\" // c\n]"
    );
    assert_eq!(
        f.format(
            r#"{"a": 1, // c
"b": 2}"#
        )
        .unwrap(),
        "{\n  \"a\": 1, // c\n  \"b\": 2\n}"
    );
}

#[test]
fn line_comment_before_comma() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // A required comma after a line comment moves to its own line.
    assert_eq!(
        f.format(
            r#"["a" // c
, "b"]"#
        )
        .unwrap(),
        "[\n  \"a\" // c\n  ,\n  \"b\"\n]"
    );
}

#[test]
fn comment_only_multiline() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(f.format("[\n  /* c */\n]").unwrap(), "[\n  /* c */\n]");
    assert_eq!(
        fmt(ALWAYS, 2, TC_NEVER).format("[/* c */]").unwrap(),
        "[\n  /* c */\n]"
    );
}

#[test]
fn multiline_block_comment_relative_indent() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // The comment moves from column 0 to column 2; the continuation line
    // gains the same two spaces.
    let input = "[\n/* multi\n   line */\n  1\n]";
    assert_eq!(
        f.format(input).unwrap(),
        "[\n  /* multi\n     line */\n  1\n]"
    );
}

#[test]
fn multiline_block_comment_crlf_normalized() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // A CRLF on the first line of a multi-line block comment is normalized
    // to LF.
    assert_eq!(
        f.format("[\n  /* first\r\n     second */\n  1\n]").unwrap(),
        "[\n  /* first\n     second */\n  1\n]"
    );
    // A CRLF on a continuation line is normalized too.
    assert_eq!(
        f.format("[\n  /* first\n     second\r\n     third */\n  1\n]")
            .unwrap(),
        "[\n  /* first\n     second\n     third */\n  1\n]"
    );
}

#[test]
fn multiline_block_comment_after_cr_comment_is_idempotent() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // A lone `\r` inside a preceding comment is comment content; it must not
    // be mistaken for a line start when the following multi-line block
    // comment's continuation-line indentation is adjusted.
    let input = "{\n  \"a\": /* k\r */ 1 /* m\r\n\tn */\n}";
    let once = f.format(input).unwrap();
    let twice = f.format(&once).unwrap();
    assert_eq!(once, twice, "formatting is not idempotent: {once:?}");
}

#[test]
fn crlf_normalized_to_lf() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(
        f.format("{\r\n  \"a\": 1,\r\n  // c\r\n  \"b\": 2\r\n}\r\n")
            .unwrap(),
        "{\n  \"a\": 1,\n  // c\n  \"b\": 2\n}"
    );
}

#[test]
fn lone_cr_outside_comments() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // A lone `\r` outside comments is a line break: the container becomes
    // multiline and the output uses LF.
    assert_eq!(f.format("[1,\r2]").unwrap(), "[\n  1,\n  2\n]");
}

#[test]
fn lone_cr_inside_comment_kept() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    // A lone `\r` inside a line comment is comment content and does not make
    // the container multiline by itself... here a newline makes it multiline
    // and the `\r` is preserved in the comment body.
    assert_eq!(
        f.format("[\n  // a\rb\n  1\n]").unwrap(),
        "[\n  // a\rb\n  1\n]"
    );
    // A lone `\r` inside a block comment is preserved too.
    assert_eq!(
        f.format("[\n  /* a\rb */\n  1\n]").unwrap(),
        "[\n  /* a\rb */\n  1\n]"
    );
}

#[test]
fn blank_lines_collapse_to_one() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(
        f.format("[\n  1,\n\n\n  2\n]").unwrap(),
        "[\n  1,\n\n  2\n]"
    );
}

#[test]
fn comment_body_punctuation_ignored() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(
        f.format("[1, /* , : [ ] { } */ 2]").unwrap(),
        "[1, /* , : [ ] { } */ 2]"
    );
}

#[test]
fn key_colon_comments_multiline() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(
        f.format("{\n  \"a\" /* k */: 1,\n  \"b\": /* v */ 2\n}")
            .unwrap(),
        "{\n  \"a\" /* k */: 1,\n  \"b\": /* v */ 2\n}"
    );
    // Line comment before the colon: the colon moves to the next line.
    assert_eq!(
        f.format("{\n  \"a\" // k\n: 1\n}").unwrap(),
        "{\n  \"a\" // k\n  : 1\n}"
    );
}

#[test]
fn root_comments() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    assert_eq!(
        f.format("// before\n{\"a\": 1}\n// after").unwrap(),
        "// before\n{\"a\": 1}\n// after"
    );
    assert_eq!(
        f.format("/* c */ {\"a\": 1}").unwrap(),
        "/* c */ {\"a\": 1}"
    );
}

#[test]
fn always_multiline_trailing_comma_with_comment() {
    let f = fmt(ALWAYS, 2, TC_ALWAYS);
    // The added trailing comma goes before the same-line trailing comment.
    assert_eq!(
        f.format(
            r#"["a" // c
]"#
        )
        .unwrap(),
        "[\n  \"a\", // c\n]"
    );
    // ... and before comments on their own lines near the closing delimiter.
    assert_eq!(
        f.format("[\n  \"a\"\n  // c\n]").unwrap(),
        "[\n  \"a\",\n  // c\n]"
    );
}

#[test]
fn no_trailing_lf() {
    let f = fmt(PRESERVE, 2, TC_PRESERVE);
    for input in ["{}", "{\n  \"a\": 1\n}", "// c\n[]"] {
        let out = f.format(input).unwrap();
        assert!(!out.ends_with('\n'), "output ends with LF: {out:?}");
    }
}

#[test]
fn indent_zero() {
    let f = fmt(PRESERVE, 0, TC_NEVER);
    assert_eq!(f.format("[\n  1,\n  2\n]").unwrap(), "[\n1,\n2\n]");
}

#[test]
fn indent_four() {
    let f = fmt(PRESERVE, 4, TC_NEVER);
    assert_eq!(f.format("{\n  \"a\": 1\n}").unwrap(), "{\n    \"a\": 1\n}");
}

#[test]
fn all_combinations_are_reparsable() {
    for line_breaks in [PRESERVE, ALWAYS] {
        for commas in [TC_PRESERVE, TC_ALWAYS, TC_NEVER] {
            for indent in [0usize, 2, 4] {
                let f = fmt(line_breaks, indent, commas);
                let inputs = [
                    "[]",
                    "{}",
                    "[1, 2, 3]",
                    "{\"a\": 1, \"b\": [true, null]}",
                    "[\n  1,\n  /* c */\n  2,\n]",
                    "// top\n{\"a\": 1}\n// bottom",
                ];
                for input in inputs {
                    let out = f.format(input).expect("format should succeed");
                    nojson::RawJson::parse_jsonc(&out).expect("output must be re-parsable");
                }
            }
        }
    }
}
