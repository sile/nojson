//! Property tests for the configurable JSONC formatter, driven by noprop.
//!
//! A random JSONC document (nested containers, comments, trailing commas, and
//! varied whitespace including CRLF, lone `\r`, tabs, and blank lines) is
//! formatted under every setting combination and checked for:
//!
//! - re-parseability of the output,
//! - idempotency (formatting the output again yields the same string),
//! - preservation of the value tree (kinds, scalar lexemes, order),
//! - preservation of comments (order and body),
//! - LF-only output (no `\r` survives).

// The suite only draws a subset of the shared harness; the roundtrip-only
// helpers (NonZero, mixed strings, ...) are dead code in this crate.
#[expect(dead_code, reason = "roundtrip-only helpers are unused here")]
mod pbt_harness;

use std::cell::Cell;

use nojson::{JsonValueKind, JsoncFormatter, JsoncLineBreaks, JsoncTrailingCommas, RawJson};
use pbt_harness::{MAX_LEN, run, sample_string_ascii_plain};

// --- JSONC document generator -----------------------------------------

const MAX_DEPTH: usize = 3;

/// Whitespace that keeps the document valid when placed between tokens.
fn sample_ws(ctx: &mut noprop::TestCaseContext) -> &'static str {
    noprop::sample_choice(
        ctx,
        &[
            "", " ", "  ", "\t", "\n", "\n\n", "\r\n", "\r", " \n ", "  \t  ",
        ],
    )
}

/// A comment body. Line comments must be followed by a newline to stay valid;
/// the caller decides where the newline goes. Block comments are single-line
/// so their body is preserved verbatim by the formatter.
fn sample_comment(ctx: &mut noprop::TestCaseContext) -> String {
    let text = noprop::sample_choice(ctx, &["a", "b", "c, : [ ] { }", "config"]);
    if noprop::sample_bool(ctx) {
        format!("// {text}")
    } else {
        format!("/* {text} */")
    }
}

/// A comment plus the whitespace that must follow it (always a single `\n`
/// after a line comment).
fn sample_comment_and_ws(ctx: &mut noprop::TestCaseContext) -> (String, String) {
    let comment = sample_comment(ctx);
    if comment.starts_with("//") {
        (comment, "\n".to_string())
    } else {
        (comment, sample_ws(ctx).to_string())
    }
}

fn sample_scalar(ctx: &mut noprop::TestCaseContext) -> String {
    match noprop::sample_weighted_index(ctx, &[1, 1, 1, 2, 2]) {
        0 => "null".to_string(),
        1 => "true".to_string(),
        2 => "false".to_string(),
        3 => noprop::sample_i32(ctx).to_string(),
        _ => {
            let s = sample_string_ascii_plain(ctx, 0, 4);
            format!("\"{s}\"")
        }
    }
}

fn sample_value(ctx: &mut noprop::TestCaseContext, depth: usize) -> String {
    if depth >= MAX_DEPTH || noprop::sample_ratio(ctx, noprop::Ratio::one_nth(3)) {
        return sample_scalar(ctx);
    }
    if noprop::sample_bool(ctx) {
        sample_array(ctx, depth)
    } else {
        sample_object(ctx, depth)
    }
}

fn sample_array(ctx: &mut noprop::TestCaseContext, depth: usize) -> String {
    let mut s = String::from("[");
    s.push_str(sample_ws(ctx));
    if noprop::sample_ratio(ctx, noprop::Ratio::one_nth(4)) {
        // Empty, possibly comment-only.
        if noprop::sample_bool(ctx) {
            let (c, ws) = sample_comment_and_ws(ctx);
            s.push_str(&c);
            s.push_str(&ws);
        }
        s.push_str(sample_ws(ctx));
        s.push(']');
        return s;
    }
    if noprop::sample_bool(ctx) {
        let (c, ws) = sample_comment_and_ws(ctx);
        s.push_str(&c);
        s.push_str(&ws);
    }
    let n = noprop::sample_usize_in(ctx, 1..=MAX_LEN);
    for i in 0..n {
        if i > 0 {
            s.push(',');
            s.push_str(sample_ws(ctx));
            if noprop::sample_bool(ctx) {
                let (c, ws) = sample_comment_and_ws(ctx);
                s.push_str(&c);
                s.push_str(&ws);
            }
        }
        s.push_str(&sample_value(ctx, depth + 1));
        if i < n - 1 && noprop::sample_bool(ctx) {
            let (c, ws) = sample_comment_and_ws(ctx);
            s.push_str(sample_ws(ctx));
            s.push_str(&c);
            s.push_str(&ws);
            s.push_str(sample_ws(ctx));
        }
    }
    if noprop::sample_bool(ctx) {
        s.push(',');
    }
    s.push_str(sample_ws(ctx));
    if noprop::sample_bool(ctx) {
        let (c, ws) = sample_comment_and_ws(ctx);
        s.push_str(&c);
        s.push_str(&ws);
    }
    s.push(']');
    s
}

fn sample_object(ctx: &mut noprop::TestCaseContext, depth: usize) -> String {
    let mut s = String::from("{");
    s.push_str(sample_ws(ctx));
    if noprop::sample_ratio(ctx, noprop::Ratio::one_nth(4)) {
        if noprop::sample_bool(ctx) {
            let (c, ws) = sample_comment_and_ws(ctx);
            s.push_str(&c);
            s.push_str(&ws);
        }
        s.push_str(sample_ws(ctx));
        s.push('}');
        return s;
    }
    if noprop::sample_bool(ctx) {
        let (c, ws) = sample_comment_and_ws(ctx);
        s.push_str(&c);
        s.push_str(&ws);
    }
    let n = noprop::sample_usize_in(ctx, 1..=MAX_LEN);
    for i in 0..n {
        if i > 0 {
            s.push(',');
            s.push_str(sample_ws(ctx));
            if noprop::sample_bool(ctx) {
                let (c, ws) = sample_comment_and_ws(ctx);
                s.push_str(&c);
                s.push_str(&ws);
            }
        }
        let key = sample_string_ascii_plain(ctx, 1, 4);
        s.push('"');
        s.push_str(&key);
        s.push('"');
        if noprop::sample_bool(ctx) {
            let (c, ws) = sample_comment_and_ws(ctx);
            s.push_str(&c);
            s.push_str(&ws);
        }
        s.push(':');
        if noprop::sample_bool(ctx) {
            let (c, ws) = sample_comment_and_ws(ctx);
            s.push_str(&c);
            s.push_str(&ws);
        }
        s.push_str(&sample_value(ctx, depth + 1));
        if i < n - 1 && noprop::sample_bool(ctx) {
            let (c, ws) = sample_comment_and_ws(ctx);
            s.push_str(sample_ws(ctx));
            s.push_str(&c);
            s.push_str(&ws);
            s.push_str(sample_ws(ctx));
        }
    }
    if noprop::sample_bool(ctx) {
        s.push(',');
    }
    s.push_str(sample_ws(ctx));
    if noprop::sample_bool(ctx) {
        let (c, ws) = sample_comment_and_ws(ctx);
        s.push_str(&c);
        s.push_str(&ws);
    }
    s.push('}');
    s
}

fn sample_document(ctx: &mut noprop::TestCaseContext) -> String {
    let mut s = String::new();
    if noprop::sample_bool(ctx) {
        let (c, ws) = sample_comment_and_ws(ctx);
        s.push_str(&c);
        s.push_str(&ws);
    }
    s.push_str(sample_ws(ctx));
    s.push_str(&sample_value(ctx, 0));
    s.push_str(sample_ws(ctx));
    if noprop::sample_bool(ctx) {
        let (c, ws) = sample_comment_and_ws(ctx);
        s.push_str(&c);
        s.push_str(&ws);
    }
    s
}

// --- Value-tree and comment extraction --------------------------------

/// Pre-order traversal comparing the kind of every value and the raw lexeme
/// of every scalar (including object keys). Container spans are compared by
/// kind only: their whitespace is normalized by the formatter.
fn value_signature(text: &str) -> Vec<(JsonValueKind, Option<String>)> {
    let (json, _) = RawJson::parse_jsonc(text).expect("input must be valid JSONC");
    fn walk(v: nojson::RawJsonValue<'_, '_>, out: &mut Vec<(JsonValueKind, Option<String>)>) {
        match v.kind() {
            JsonValueKind::Array | JsonValueKind::Object => {
                out.push((v.kind(), None));
            }
            _ => out.push((v.kind(), Some(v.as_raw_str().to_string()))),
        }
        match v.kind() {
            JsonValueKind::Array => {
                for e in v.to_array().expect("array") {
                    walk(e, out);
                }
            }
            JsonValueKind::Object => {
                for (k, val) in v.to_object().expect("object") {
                    walk(k, out);
                    walk(val, out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(json.value(), &mut out);
    out
}

/// Comment bodies in source order.
fn comment_bodies(text: &str) -> Vec<String> {
    let (_, comments) = RawJson::parse_jsonc(text).expect("input must be valid JSONC");
    comments
        .iter()
        .map(|r| text[r.clone()].to_string())
        .collect()
}

// --- Properties -------------------------------------------------------

#[test]
fn jsonc_format_preserves_structure_and_is_idempotent() -> noprop::TestResult {
    let comment_cases = Cell::new(0usize);
    let crlf_cases = Cell::new(0usize);
    let blank_line_cases = Cell::new(0usize);
    let trailing_comma_cases = Cell::new(0usize);
    run(|ctx| {
        let input = sample_document(ctx);
        let input_comments = comment_bodies(&input);
        let input_signature = value_signature(&input);
        for line_breaks in [JsoncLineBreaks::Preserve, JsoncLineBreaks::Always] {
            for commas in [
                JsoncTrailingCommas::Preserve,
                JsoncTrailingCommas::AlwaysMultiline,
                JsoncTrailingCommas::Never,
            ] {
                for indent in [0usize, 2, 4] {
                    let formatter = JsoncFormatter {
                        indent_size: indent,
                        line_breaks,
                        trailing_commas: commas,
                    };
                    let output = formatter.format(&input).expect("format must succeed");
                    RawJson::parse_jsonc(&output).expect("output must be re-parsable");
                    let again = formatter.format(&output).expect("second format");
                    assert_eq!(output, again, "formatting is not idempotent for {input:?}");
                    assert_eq!(
                        value_signature(&output),
                        input_signature,
                        "value tree changed for {input:?}"
                    );
                    assert_eq!(
                        comment_bodies(&output),
                        input_comments,
                        "comments changed for {input:?}"
                    );
                    assert!(!output.contains('\r'), "output contains a CR for {input:?}");
                }
            }
        }
        if input.contains("//") || input.contains("/*") {
            comment_cases.set(comment_cases.get() + 1);
        }
        if input.contains("\r\n") {
            crlf_cases.set(crlf_cases.get() + 1);
        }
        if input.contains("\n\n") || input.contains("\r\r") || input.contains("\r\n\r\n") {
            blank_line_cases.set(blank_line_cases.get() + 1);
        }
        if input.contains(",\n") || input.contains(",\r") || input.contains(", ") {
            // The document may or may not have a trailing comma; count cases
            // where the AlwaysMultiline policy can add one.
            trailing_comma_cases.set(trailing_comma_cases.get() + 1);
        }
        Ok(())
    })?;
    assert!(comment_cases.get() > 0, "no case generated a comment");
    assert!(crlf_cases.get() > 0, "no case generated CRLF");
    assert!(blank_line_cases.get() > 0, "no case generated a blank line");
    assert!(
        trailing_comma_cases.get() > 0,
        "no case exercised trailing-comma handling"
    );
    Ok(())
}
