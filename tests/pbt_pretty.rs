//! Pretty-print roundtrip property tests for nojson, driven by noprop.
//!
//! `nojson::JsonFormatter::set_indent_size` / `set_spacing` route output
//! through a separate code path (indentation and member separators)
//! from the compact formatter exercised by `pbt_roundtrip.rs`. Every
//! test picks a value, pretty-prints it under random settings, parses
//! the result back, and asserts the parsed value equals the original.

// The pretty suite only draws a subset of the shared harness; the
// roundtrip-only helpers (NonZero, mixed strings, ...) are dead code
// in this crate.
#[expect(dead_code, reason = "roundtrip-only helpers are unused here")]
mod pbt_harness;

use std::cell::Cell;
use std::collections::BTreeMap;

use pbt_harness::{MAX_LEN, needs_escape, run, sample_len, sample_string_arbitrary, sample_vec};

/// Sample a pretty-print setting pair: an indent size covering the
/// compact (`0`) through the common (`2`, `4`) and large (`8`) values,
/// plus whether spacing between tokens is enabled.
fn sample_pretty_settings(ctx: &mut noprop::TestCaseContext) -> (usize, bool) {
    let indent = noprop::sample_choice(ctx, &[0usize, 1, 2, 4, 8]);
    let spacing = noprop::sample_bool(ctx);
    (indent, spacing)
}

#[test]
fn pretty_roundtrip_nested() -> noprop::TestResult {
    let spacing_cases = Cell::new(0usize);
    let indented_cases = Cell::new(0usize);
    run(|ctx| {
        let (indent, spacing) = sample_pretty_settings(ctx);
        let v = sample_vec(ctx, |ctx| {
            let n = sample_len(ctx, MAX_LEN);
            let mut m = BTreeMap::new();
            for _ in 0..n {
                m.insert(
                    sample_string_arbitrary(ctx),
                    sample_vec(ctx, noprop::sample_i32),
                );
            }
            m
        });
        let text = nojson::json(|f| {
            f.set_indent_size(indent);
            f.set_spacing(spacing);
            f.value(&v)
        })
        .to_string();
        let parsed: nojson::Json<Vec<BTreeMap<String, Vec<i32>>>> = text.parse()?;
        assert_eq!(parsed.0, v);
        if spacing {
            spacing_cases.set(spacing_cases.get() + 1);
        }
        if indent > 0 {
            indented_cases.set(indented_cases.get() + 1);
        }
        Ok(())
    })?;
    assert!(
        spacing_cases.get() > 0,
        "no case pretty-printed with set_spacing(true)"
    );
    assert!(
        indented_cases.get() > 0,
        "no case pretty-printed with a non-zero indent size"
    );
    Ok(())
}

#[test]
fn pretty_roundtrip_string() -> noprop::TestResult {
    let escape_cases = Cell::new(0usize);
    run(|ctx| {
        let (indent, spacing) = sample_pretty_settings(ctx);
        let s = sample_string_arbitrary(ctx);
        let text = nojson::json(|f| {
            f.set_indent_size(indent);
            f.set_spacing(spacing);
            f.value(&s)
        })
        .to_string();
        let parsed: nojson::Json<String> = text.parse()?;
        assert_eq!(parsed.0, s);
        if s.chars().any(needs_escape) {
            escape_cases.set(escape_cases.get() + 1);
        }
        Ok(())
    })?;
    assert!(
        escape_cases.get() > 0,
        "no case pretty-printed a string that requires JSON escaping"
    );
    Ok(())
}
