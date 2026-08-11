//! Property tests for the parser's nesting-depth cap.
//!
//! Generates a random depth in `1..=2 * MAX_NESTING_DEPTH` and asserts:
//!
//! - depths in `1..=MAX_NESTING_DEPTH` parse successfully;
//! - depths in `MAX_NESTING_DEPTH + 1..=2 * MAX_NESTING_DEPTH` are rejected
//!   with `JsonParseError::InvalidValue` whose message contains
//!   `"nesting depth exceeded"`.
//!
//! Complements the boundary-value unit tests in `test_parse.rs` by covering
//! depths that the enumerated cases (MAX and MAX + 1) do not touch, guarding
//! against future off-by-one regressions in the cap logic.

// This suite only pulls `run` from the shared harness; the roundtrip helpers
// (NonZero, mixed strings, ...) are dead code here.
#[expect(dead_code, reason = "roundtrip-only helpers are unused here")]
mod pbt_harness;

use nojson::{JsonParseError, JsonValueKind, RawJson};
use pbt_harness::run;

const MAX_TEST_DEPTH: usize = nojson::MAX_NESTING_DEPTH * 2;

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
    // `{"k":` (5 bytes) per level plus a `null` leaf and one `}` per level.
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

fn assert_depth_property(depth: usize, expected_kind: JsonValueKind, text: &str) {
    match RawJson::parse(text) {
        Ok(_) => assert!(
            depth <= nojson::MAX_NESTING_DEPTH,
            "depth {depth}: parse unexpectedly succeeded past the cap",
        ),
        Err(e) => {
            assert!(
                depth > nojson::MAX_NESTING_DEPTH,
                "depth {depth}: parse unexpectedly failed: {e:?}",
            );
            assert!(
                matches!(e, JsonParseError::InvalidValue { .. }),
                "depth {depth}: expected InvalidValue, got {e:?}",
            );
            assert_eq!(e.kind(), Some(expected_kind), "depth {depth}");
            let msg = e.to_string();
            assert!(
                msg.contains("nesting depth exceeded"),
                "depth {depth}: message does not mention nesting depth: {msg}",
            );
        }
    }
}

#[test]
fn depth_property_arrays() -> noprop::TestResult {
    run(|ctx| {
        let depth = noprop::sample_usize_in(ctx, 1..=MAX_TEST_DEPTH);
        let text = nested_arrays(depth);
        assert_depth_property(depth, JsonValueKind::Array, &text);
        Ok(())
    })
}

#[test]
fn depth_property_objects() -> noprop::TestResult {
    run(|ctx| {
        let depth = noprop::sample_usize_in(ctx, 1..=MAX_TEST_DEPTH);
        let text = nested_objects(depth);
        assert_depth_property(depth, JsonValueKind::Object, &text);
        Ok(())
    })
}
