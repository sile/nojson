//! Behavior checks for the binaries under `examples/`.
//!
//! Each test spawns the example via `cargo run --example <name>`, feeds input
//! on stdin, and inspects the output. The first run per example builds it; the
//! rest reuse the cached binary.

use std::io::Write;
use std::process::{Command, Stdio};

struct Output {
    stdout: String,
    stderr: String,
    success: bool,
}

fn run_example(name: &str, input: &str) -> Output {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut child = Command::new(cargo)
        .args(["run", "--quiet", "--example", name])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn cargo");
    child
        .stdin
        .as_mut()
        .expect("stdin piped")
        .write_all(input.as_bytes())
        .expect("write stdin");
    let out = child.wait_with_output().expect("wait cargo");
    Output {
        stdout: String::from_utf8(out.stdout).expect("utf8 stdout"),
        stderr: String::from_utf8(out.stderr).expect("utf8 stderr"),
        success: out.status.success(),
    }
}

#[test]
fn parse_error_accepts_valid_json() {
    let out = run_example("parse_error", r#"{"a": 1, "b": [true, null]}"#);
    assert!(out.success, "stderr: {}", out.stderr);
    assert!(out.stdout.contains("valid JSON"), "stdout: {}", out.stdout);
}

#[test]
fn parse_error_reports_line_column_and_caret() {
    let out = run_example("parse_error", "{\n  \"a\": 1,\n  \"b\":\n}");
    assert!(!out.success);
    assert!(out.stderr.contains("error:"), "stderr: {}", out.stderr);
    // Error is on line 4 (the closing `}` arrives where a value was expected).
    assert!(out.stderr.contains("4 |"), "stderr: {}", out.stderr);
    assert!(out.stderr.contains("^ here"), "stderr: {}", out.stderr);
}

#[test]
fn jsonc_pretty_formats_object_multiline() {
    let out = run_example("jsonc_pretty", r#"{"a":1,"b":2}"#);
    assert!(out.success, "stderr: {}", out.stderr);
    assert_eq!(out.stdout, "{\n  \"a\": 1,\n  \"b\": 2\n}\n");
}

#[test]
fn jsonc_pretty_preserves_comments_on_own_lines() {
    let out = run_example("jsonc_pretty", r#"{"a":1,/*c*/"b":2}"#);
    assert!(out.success, "stderr: {}", out.stderr);
    assert_eq!(out.stdout, "{\n  \"a\": 1,\n  /*c*/\n  \"b\": 2\n}\n");
}

#[test]
fn jsonc_pretty_keeps_empty_containers_compact() {
    let out = run_example("jsonc_pretty", r#"{"a":[],"b":{}}"#);
    assert!(out.success, "stderr: {}", out.stderr);
    assert_eq!(out.stdout, "{\n  \"a\": [],\n  \"b\": {}\n}\n");
}

#[test]
fn jsonc_pretty_rejects_invalid_input() {
    let out = run_example("jsonc_pretty", "{not valid}");
    assert!(!out.success);
    assert!(!out.stderr.is_empty());
}
