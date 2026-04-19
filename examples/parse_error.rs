//! Print JSON parse errors with line/column context.
//!
//! Reads JSON from stdin and validates it with [`nojson::RawJson::parse`].
//! On failure, it renders the error along with the offending source line and
//! a caret pointing at the byte position reported by
//! [`nojson::JsonParseError::position`].
//!
//! # Example
//!
//! Valid JSON on stdin:
//!
//! ```console
//! $ echo '{"a": 1, "b": [true, null]}' | cargo run --example parse_error
//! valid JSON
//! ```
//!
//! Invalid JSON — the error is printed to stderr with the offending line and
//! a caret under the reported byte position:
//!
//! ```console
//! $ printf '{\n  "a": 1,\n  "b":\n}\n' | cargo run --example parse_error
//! error: unexpected char while parsing Object at byte position 19
//!    4 | }
//!      | ^ here
//! ```

use std::io::{self, Read};
use std::num::NonZeroUsize;
use std::process::ExitCode;

use nojson::{JsonParseError, RawJson};

fn main() -> ExitCode {
    let mut text = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut text) {
        eprintln!("failed to read stdin: {e}");
        return ExitCode::FAILURE;
    }

    match RawJson::parse(&text) {
        Ok(_) => {
            println!("valid JSON");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", format_error(&text, &e));
            ExitCode::FAILURE
        }
    }
}

fn format_error(text: &str, error: &JsonParseError) -> String {
    let (line_num, column) = error
        .get_line_and_column_numbers(text)
        .unwrap_or((NonZeroUsize::MIN, NonZeroUsize::MIN));
    let line = error.get_line(text).unwrap_or("");
    let (snippet, col) = window_around_column(line, column.get());

    format!(
        "error: {error}\n\
         {line_num:>4} | {snippet}\n\
         {empty:>4} | {caret:>col$} here",
        empty = "",
        caret = "^",
    )
}

// Trim `line` to a window centered on `column` (1-based), returning the trimmed
// text and the adjusted column within it. Keeps long lines readable.
fn window_around_column(line: &str, column: usize) -> (String, usize) {
    const MAX_CHARS: usize = 80;
    let chars: Vec<char> = line.chars().collect();
    let half = MAX_CHARS / 2;

    let pos = column.saturating_sub(1).min(chars.len());
    let start = pos.saturating_sub(half);
    let end = (pos + half + 1).min(chars.len());

    let mut out = String::new();
    let mut new_col = pos - start + 1;
    if start > 0 {
        out.push_str("...");
        new_col += 3;
    }
    out.extend(chars[start..end].iter());
    if end < chars.len() {
        out.push_str("...");
    }
    (out, new_col)
}
