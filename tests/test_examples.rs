//! Behavior checks for the binaries under `examples/`.
//!
//! The examples are built once with `cargo build --examples`; each
//! test then spawns the built binary directly. Spawning `cargo run`
//! once per test was flaky: concurrent `cargo run` processes contend
//! for the shared target directory and some get killed silently
//! (SIGKILL with empty stderr), failing tests at random.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;

struct Output {
    stdout: String,
    stderr: String,
    success: bool,
}

/// Builds every example exactly once. Tests running in parallel block
/// in `get_or_init` until the first build completes; if the build
/// panics, the lock stays uninitialized so a later test retries.
static EXAMPLE_BUILD: OnceLock<()> = OnceLock::new();

fn build_examples() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let status = Command::new(cargo)
        .args(["build", "--examples", "--quiet"])
        .status()
        .expect("failed to spawn cargo build");
    assert!(
        status.success(),
        "cargo build --examples failed with {status}"
    );
}

/// Resolve the path of a built example binary, honouring a custom
/// `CARGO_TARGET_DIR` when set.
fn example_bin_path(name: &str) -> PathBuf {
    let target_dir = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
    let bin = target_dir.join("debug/examples").join(name);
    #[cfg(windows)]
    let bin = bin.with_extension("exe");
    bin
}

fn run_example(name: &str, input: &str) -> Output {
    EXAMPLE_BUILD.get_or_init(build_examples);
    let mut child = Command::new(example_bin_path(name))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn example binary");
    child
        .stdin
        .as_mut()
        .expect("stdin piped")
        .write_all(input.as_bytes())
        .expect("write stdin");
    let out = child.wait_with_output().expect("wait example binary");
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
