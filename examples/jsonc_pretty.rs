//! Pretty-print JSONC (JSON with comments and trailing commas).
//!
//! Reads JSONC from stdin, validates it with [`nojson::RawJson::parse_jsonc`],
//! and writes it back with 2-space indentation. Comments are preserved on
//! their own lines just before the value they precede.
//!
//! This is a minimal demo of [`nojson::RawJson::parse_jsonc`] and the
//! [`nojson::RawJsonValue`] traversal API. For a production-quality JSONC
//! formatter that preserves trailing / inline comments, trailing commas, and
//! blank lines, see the `jcfmt` crate.
//!
//! # Example
//!
//! Input (compact JSONC with a block comment and a trailing comma):
//!
//! ```jsonc
//! {"name":"example",/* config */"config":{"debug":true,"port":8080},"tags":["a","b",]}
//! ```
//!
//! Output:
//!
//! ```jsonc
//! {
//!   "name": "example",
//!   /* config */
//!   "config": {
//!     "debug": true,
//!     "port": 8080
//!   },
//!   "tags": [
//!     "a",
//!     "b"
//!   ]
//! }
//! ```
//!
//! Run with:
//!
//! ```console
//! $ cat example.jsonc | cargo run --example jsonc_pretty
//! ```

use std::io::{self, Read, Write};
use std::ops::Range;

use nojson::{JsonValueKind, RawJson, RawJsonValue};

const INDENT: &str = "  ";

fn main() -> io::Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    let (json, comments) = RawJson::parse_jsonc(&text)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e}")))?;

    let mut p = Printer {
        text: &text,
        comments: &comments,
        next_comment: 0,
        level: 0,
        out: String::new(),
    };
    p.flush_comments_before(json.value().position());
    p.value(json.value());
    p.flush_comments_before(text.len());

    writeln!(io::stdout(), "{}", p.out.trim_end_matches('\n'))
}

struct Printer<'a> {
    text: &'a str,
    comments: &'a [Range<usize>],
    next_comment: usize,
    level: usize,
    out: String,
}

impl Printer<'_> {
    fn indent(&mut self) {
        for _ in 0..self.level {
            self.out.push_str(INDENT);
        }
    }

    fn newline_indent(&mut self) {
        self.out.push('\n');
        self.indent();
    }

    // Emit every comment whose start offset is before `pos`, each on its own
    // indented line. Multi-line block comments keep their line breaks.
    fn flush_comments_before(&mut self, pos: usize) {
        while let Some(range) = self.comments.get(self.next_comment) {
            if range.start >= pos {
                return;
            }
            if !self.out.is_empty() {
                self.newline_indent();
            }
            let mut lines = self.text[range.clone()].lines();
            if let Some(first) = lines.next() {
                self.out.push_str(first.trim_end());
            }
            for line in lines {
                self.newline_indent();
                self.out.push_str(line.trim());
            }
            self.next_comment += 1;
        }
    }

    fn value(&mut self, v: RawJsonValue<'_, '_>) {
        match v.kind() {
            JsonValueKind::Array => self.array(v),
            JsonValueKind::Object => self.object(v),
            _ => self.out.push_str(v.as_raw_str()),
        }
    }

    fn array(&mut self, v: RawJsonValue<'_, '_>) {
        let mut items = v.to_array().expect("array");
        let Some(first) = items.next() else {
            self.out.push_str("[]");
            return;
        };
        self.out.push('[');
        self.level += 1;
        self.flush_comments_before(first.position());
        self.newline_indent();
        self.value(first);
        for item in items {
            self.out.push(',');
            self.flush_comments_before(item.position());
            self.newline_indent();
            self.value(item);
        }
        let close = v.position() + v.as_raw_str().len() - 1;
        self.flush_comments_before(close);
        self.level -= 1;
        self.newline_indent();
        self.out.push(']');
    }

    fn object(&mut self, v: RawJsonValue<'_, '_>) {
        let mut members = v.to_object().expect("object");
        let Some((k, val)) = members.next() else {
            self.out.push_str("{}");
            return;
        };
        self.out.push('{');
        self.level += 1;
        self.emit_member(k, val);
        for (k, val) in members {
            self.out.push(',');
            self.emit_member(k, val);
        }
        let close = v.position() + v.as_raw_str().len() - 1;
        self.flush_comments_before(close);
        self.level -= 1;
        self.newline_indent();
        self.out.push('}');
    }

    fn emit_member(&mut self, k: RawJsonValue<'_, '_>, val: RawJsonValue<'_, '_>) {
        self.flush_comments_before(k.position());
        self.newline_indent();
        self.out.push_str(k.as_raw_str());
        self.out.push_str(": ");
        self.value(val);
    }
}
