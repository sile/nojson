//! Configurable JSONC formatting ([`JsoncFormatter`]).

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::{JsonParseError, JsonValueKind, RawJson, RawJsonValue};

/// Settings for the JSONC formatter.
///
/// [`JsoncFormatter::format`] guarantees that accepted input is re-parsable,
/// that formatting is idempotent, and that comments, scalar lexemes, and
/// member order are preserved.
///
/// # Formatting rules
///
/// `format` re-emits the input with normalized whitespace and line breaks.
/// How line breaks inside arrays and objects are chosen (`line_breaks`) and
/// how trailing commas are handled (`trailing_commas`) are configured
/// separately; see [`JsoncLineBreaks`] and [`JsoncTrailingCommas`]. The rules
/// below apply to every combination of those settings.
///
/// ## Line-break characters
///
/// The output is LF-only. CRLF and a lone `\r` outside comments are converted
/// to LF, and the output never ends with a line break. Consecutive blank
/// lines collapse into one.
///
/// ```jsonc
/// [
///   1,
///
///
///   2
/// ]
/// ->
/// [
///   1,
///
///   2
/// ]
/// ```
///
/// ## Spacing and indentation
///
/// In single-line containers, whitespace is normalized: one space after `,`
/// and `:`, none directly inside `[`/`{` or before `]`/`}`, and comments set
/// off by one space. In multi-line containers, every element and member is
/// placed on its own line, indented by `indent_size` spaces per nesting
/// level (`indent_size: 0` disables indentation). `indent_size` has no
/// effect on single-line containers, and a long single-line container is
/// never wrapped onto multiple lines.
///
/// ```jsonc
/// {"a":1,"b":2} -> {"a": 1, "b": 2}
/// ```
///
/// ## Comments
///
/// Comments keep their body, their order, and their position relative to the
/// surrounding elements. A `//` comment ends its line, so whatever follows it
/// moves to a new line; a space is added in front of it when the input had
/// none. The body of a multi-line `/* */` comment is kept verbatim; only the
/// leading spaces of its continuation lines are adjusted to the comment's new
/// column (a leftward move can collapse them to none).
///
/// ```jsonc
/// {"a" /* k */: 1} -> {"a" /* k */: 1}
///
/// [1// c
/// ]
/// ->
/// [
///   1 // c
/// ]
///
/// [
/// /* multi
///    line */
///   1
/// ]
/// ->
/// [
///   /* multi
///      line */
///   1
/// ]
/// ```
///
/// # Note
///
/// This formatter is a convenience for common JSONC layouts, not a general
/// pretty printer: it does not reproduce every possible arrangement of
/// comments or whitespace. If the fixed rules do not fit a layout, build a
/// custom formatter with [`RawJson::parse_jsonc`](crate::RawJson::parse_jsonc),
/// [`RawJsonValue`](crate::RawJsonValue), the original source text, and the
/// comment byte ranges instead.
// No `Default` impl: all three fields must be chosen explicitly by the caller
// so that no layout policy is silently preferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsoncFormatter {
    /// Number of ASCII spaces emitted per indentation level.
    pub indent_size: usize,
    /// How line breaks inside arrays and objects are chosen.
    pub line_breaks: JsoncLineBreaks,
    /// How trailing commas are handled.
    pub trailing_commas: JsoncTrailingCommas,
}

/// Line-break policy for arrays and objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsoncLineBreaks {
    /// Keep each array and object on a single line or on multiple lines
    /// according to whether the input had a physical line break inside it.
    ///
    /// Layout is decided from the input alone: a single-line container stays
    /// single-line even when a sibling container is multi-line. A multi-line
    /// child also keeps its ancestor containers multi-line, because its line
    /// break lies inside their span.
    ///
    /// The examples below assume
    /// [`JsoncTrailingCommas::Preserve`](JsoncTrailingCommas::Preserve) and an
    /// indent size of 2.
    ///
    /// # Examples
    ///
    /// ```jsonc
    /// // single-line input stays single-line
    /// [1, 2] -> [1, 2]
    ///
    /// // empty containers stay single-line
    /// [] -> []
    /// {} -> {}
    ///
    /// // multi-line input stays multi-line
    /// [
    /// 1, 2
    /// ]
    /// ->
    /// [
    ///   1,
    ///   2
    /// ]
    ///
    /// // single-line siblings of a multi-line container stay single-line
    /// {
    ///   "left": [
    ///     1,
    ///     2
    ///   ],
    ///   "right": [3, 4], "nested": {"items": [5, 6]}
    /// }
    /// ->
    /// {
    ///   "left": [
    ///     1,
    ///     2
    ///   ],
    ///   "right": [3, 4],
    ///   "nested": {"items": [5, 6]}
    /// }
    /// ```
    Preserve,
    /// Put every array and object that contains an element, a member, or a
    /// comment onto multiple lines. Truly empty `[]` and `{}` stay single-line.
    ///
    /// The examples below assume
    /// [`JsoncTrailingCommas::Preserve`](JsoncTrailingCommas::Preserve) and an
    /// indent size of 2.
    ///
    /// # Examples
    ///
    /// ```jsonc
    /// // containers with content are expanded
    /// [1, 2]
    /// ->
    /// [
    ///   1,
    ///   2
    /// ]
    ///
    /// // empty containers stay single-line
    /// [] -> []
    /// {} -> {}
    ///
    /// // comment-only containers are expanded, not treated as empty
    /// [/* c */]
    /// ->
    /// [
    ///   /* c */
    /// ]
    /// ```
    Always,
}

/// Trailing-comma policy, applied identically to arrays and objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsoncTrailingCommas {
    /// Keep a trailing comma from the input in place, including its position
    /// relative to any trailing comments (the input order of the comma and
    /// the comments is not swapped).
    ///
    /// The examples below assume
    /// [`JsoncLineBreaks::Preserve`](JsoncLineBreaks::Preserve) and an indent
    /// size of 2.
    ///
    /// # Examples
    ///
    /// ```jsonc
    /// [1, 2]          -> [1, 2]
    /// [1, 2,]         -> [1, 2,]
    /// [1, 2, /* c */] -> [1, 2, /* c */]  // comma before the comment
    /// [1, 2 /* c */,] -> [1, 2 /* c */,]  // comma after the comment
    /// ```
    Preserve,
    /// Add a trailing comma to multi-line containers that contain at least
    /// one element or member; remove it from single-line containers. Empty
    /// and comment-only containers get no trailing comma.
    ///
    /// The examples below assume
    /// [`JsoncLineBreaks::Preserve`](JsoncLineBreaks::Preserve) and an indent
    /// size of 2.
    ///
    /// # Examples
    ///
    /// ```jsonc
    /// // multi-line container: comma is added
    /// [
    ///   1,
    ///   2
    /// ]
    /// ->
    /// [
    ///   1,
    ///   2,
    /// ]
    ///
    /// // single-line container: comma is removed
    /// [1, 2,] -> [1, 2]
    /// ```
    AlwaysMultiline,
    /// Remove trailing commas from arrays and objects.
    ///
    /// The examples below assume
    /// [`JsoncLineBreaks::Preserve`](JsoncLineBreaks::Preserve) and an indent
    /// size of 2.
    ///
    /// # Examples
    ///
    /// ```jsonc
    /// [1, 2,] -> [1, 2]
    ///
    /// [
    ///   1,
    ///   2,
    /// ]
    /// ->
    /// [
    ///   1,
    ///   2
    /// ]
    /// ```
    Never,
}

impl JsoncFormatter {
    /// Parses `text` as JSONC and returns it reformatted according to `self`.
    ///
    /// The input is validated with
    /// [`RawJson::parse_jsonc`](crate::RawJson::parse_jsonc); invalid JSONC is
    /// returned as a [`JsonParseError`]. The returned string never ends with a
    /// line break and uses LF as its only line-break character.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), nojson::JsonParseError> {
    /// let formatter = nojson::JsoncFormatter {
    ///     indent_size: 2,
    ///     line_breaks: nojson::JsoncLineBreaks::Always,
    ///     trailing_commas: nojson::JsoncTrailingCommas::AlwaysMultiline,
    /// };
    ///
    /// let output = formatter.format(r#"{"name":"example",/* config */"tags":["a","b"]}"#)?;
    /// assert_eq!(
    ///     output,
    ///     r#"{
    ///   "name": "example", /* config */
    ///   "tags": [
    ///     "a",
    ///     "b",
    ///   ],
    /// }"#
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn format(&self, text: &str) -> Result<String, JsonParseError> {
        let (json, comments) = RawJson::parse_jsonc(text)?;
        let root = json.value();
        let root_span = root.position()..root.position() + root.as_raw_str().len();

        let tree = build_node(root);
        let mut fmt = Formatter {
            text,
            comments: &comments,
            settings: *self,
            out: Out::default(),
            level: 0,
        };

        let first_after_root = comments.partition_point(|c| c.end <= root_span.start);
        let leading: Vec<Item> = comments[..first_after_root]
            .iter()
            .map(|c| Item::Comment(c.clone()))
            .collect();
        let (last_line, last_content_end) = fmt.emit_gap_items(0, &leading, true, false);
        let root_line = fmt.line_index(0, root_span.start);
        if root_line > last_line {
            fmt.out.push('\n');
            if fmt.has_blank_line(last_content_end, root_span.start) {
                fmt.out.push('\n');
            }
        } else if fmt.out.column() != 0 {
            fmt.out.push(' ');
        }
        fmt.emit_value(&tree);

        let trailing: Vec<Item> = comments[first_after_root..]
            .iter()
            .filter(|c| c.start >= root_span.end)
            .map(|c| Item::Comment(c.clone()))
            .collect();
        fmt.emit_gap_items(root_span.end, &trailing, false, false);

        Ok(fmt.out.into_string())
    }
}

// --- Internal model ---------------------------------------------------

/// A value in the JSONC document, with its byte span in the original text and
/// its direct children. Scalar lexemes are re-emitted verbatim from the source.
struct Node {
    kind: JsonValueKind,
    span: Range<usize>,
    children: Children,
}

enum Children {
    Leaf,
    Array(Vec<Node>),
    Object(Vec<(Node, Node)>),
}

fn build_node(value: RawJsonValue<'_, '_>) -> Node {
    let kind = value.kind();
    let span = value.position()..value.position() + value.as_raw_str().len();
    let children = match kind {
        JsonValueKind::Array => {
            Children::Array(value.to_array().expect("array").map(build_node).collect())
        }
        JsonValueKind::Object => Children::Object(
            value
                .to_object()
                .expect("object")
                .map(|(key, val)| (build_node(key), build_node(val)))
                .collect(),
        ),
        _ => Children::Leaf,
    };
    Node {
        kind,
        span,
        children,
    }
}

/// An item found in a gap between two structural tokens.
#[derive(Clone)]
enum Item {
    Comment(Range<usize>),
    Comma(usize),
    Colon(usize),
}

impl Item {
    fn position(&self) -> usize {
        match self {
            Item::Comment(c) => c.start,
            Item::Comma(p) | Item::Colon(p) => *p,
        }
    }
}

/// Output buffer that tracks the byte column of the current line.
#[derive(Default)]
struct Out {
    s: String,
    col: usize,
}

impl Out {
    fn push_str(&mut self, s: &str) {
        if let Some(idx) = s.rfind('\n') {
            self.col = s.len() - idx - 1;
        } else {
            self.col += s.len();
        }
        self.s.push_str(s);
    }

    fn push(&mut self, c: char) {
        if c == '\n' {
            self.col = 0;
        } else {
            self.col += 1;
        }
        self.s.push(c);
    }

    fn column(&self) -> usize {
        self.col
    }

    fn into_string(self) -> String {
        self.s
    }
}

struct Formatter<'a> {
    text: &'a str,
    comments: &'a [Range<usize>],
    settings: JsoncFormatter,
    out: Out,
    level: usize,
}

impl<'a> Formatter<'a> {
    /// Returns `true` when `node` (a container) is multi-line in the input
    /// according to the configured line-break policy.
    fn initial_multiline(&self, node: &Node) -> bool {
        match self.settings.line_breaks {
            JsoncLineBreaks::Preserve => self.has_physical_newline(&node.span),
            JsoncLineBreaks::Always => {
                let has_children = match &node.children {
                    Children::Array(elems) => !elems.is_empty(),
                    Children::Object(members) => !members.is_empty(),
                    Children::Leaf => false,
                };
                has_children || self.has_comment_in_gaps(node)
            }
        }
    }

    fn emit_value(&mut self, node: &Node) {
        match node.kind {
            JsonValueKind::Array | JsonValueKind::Object => {
                if self.initial_multiline(node) {
                    match &node.children {
                        Children::Array(_) => self.emit_multiline_array(node),
                        Children::Object(_) => self.emit_multiline_object(node),
                        Children::Leaf => unreachable!(),
                    }
                } else {
                    self.emit_single_line(node);
                }
            }
            _ => {
                let span = &node.span;
                self.out.push_str(&self.text[span.clone()]);
            }
        }
    }

    // --- Gap scanning ------------------------------------------------

    /// Returns the items (comments and structural punctuation) inside
    /// `[start, end)` in source order.
    fn scan_gap(&self, start: usize, end: usize) -> Vec<Item> {
        let mut items = Vec::new();
        let mut p = start;
        let mut ci = self.comments.partition_point(|c| c.start < start);
        while p < end {
            if let Some(c) = self.comments.get(ci)
                && c.start == p
            {
                items.push(Item::Comment(c.clone()));
                p = c.end;
                ci += 1;
                continue;
            }
            match self.text.as_bytes()[p] {
                b',' => {
                    items.push(Item::Comma(p));
                    p += 1;
                }
                b':' => {
                    items.push(Item::Colon(p));
                    p += 1;
                }
                _ => p += 1,
            }
        }
        items
    }

    /// Number of line breaks in `[start, pos)`. A `\n` anywhere (including
    /// inside a block comment) and a lone `\r` outside comments each count as
    /// one break; a lone `\r` inside a comment is comment content and does not
    /// count.
    fn line_index(&self, start: usize, pos: usize) -> usize {
        let text = self.text.as_bytes();
        let mut lines = 0;
        let mut p = start;
        let mut ci = self.comments.partition_point(|c| c.end <= start);
        while p < pos {
            if let Some(c) = self.comments.get(ci)
                && c.start == p
            {
                let mut q = p;
                while q < c.end {
                    if text[q] == b'\n' {
                        lines += 1;
                    }
                    q += 1;
                }
                p = c.end;
                ci += 1;
                continue;
            }
            match text[p] {
                b'\n' => {
                    lines += 1;
                    p += 1;
                }
                b'\r' => {
                    if text.get(p + 1) == Some(&b'\n') {
                        lines += 1;
                        p += 2;
                    } else {
                        lines += 1;
                        p += 1;
                    }
                }
                _ => p += 1,
            }
        }
        lines
    }

    /// Returns `true` when `[start, pos)` contains a blank line: two line
    /// breaks separated only by spaces and tabs, with no comment content in
    /// between. Line breaks inside comments are ignored.
    fn has_blank_line(&self, start: usize, pos: usize) -> bool {
        let text = self.text.as_bytes();
        let mut p = start;
        let mut ci = self.comments.partition_point(|c| c.end <= start);
        let mut prev_break = false;
        let mut line_has_content = false;
        while p < pos {
            if let Some(c) = self.comments.get(ci)
                && c.start == p
            {
                line_has_content = true;
                p = c.end;
                ci += 1;
                continue;
            }
            match text[p] {
                b'\n' | b'\r' => {
                    if prev_break && !line_has_content {
                        return true;
                    }
                    prev_break = true;
                    line_has_content = false;
                    p += if text[p] == b'\r' && text.get(p + 1) == Some(&b'\n') {
                        2
                    } else {
                        1
                    };
                }
                b' ' | b'\t' => p += 1,
                _ => {
                    line_has_content = true;
                    p += 1;
                }
            }
        }
        false
    }

    /// Returns `true` when the container span contains a physical line break:
    /// any `\n` (including inside block comments) or any `\r` outside comments.
    fn has_physical_newline(&self, span: &Range<usize>) -> bool {
        let text = self.text.as_bytes();
        let mut p = span.start;
        let mut ci = self.comments.partition_point(|c| c.end <= span.start);
        while p < span.end {
            if let Some(c) = self.comments.get(ci)
                && c.start == p
            {
                let mut q = p;
                while q < c.end {
                    if text[q] == b'\n' {
                        return true;
                    }
                    q += 1;
                }
                p = c.end;
                ci += 1;
                continue;
            }
            match text[p] {
                b'\n' | b'\r' => return true,
                _ => p += 1,
            }
        }
        false
    }

    /// Returns `true` when the container has a comment in one of its own gaps
    /// (not nested inside a child value).
    ///
    /// Child spans and comments are both in source order, so each comment is
    /// matched against at most one candidate child with a binary search
    /// (`partition_point`); scanning every child per comment would be
    /// quadratic in the number of children and comments.
    fn has_comment_in_gaps(&self, node: &Node) -> bool {
        let mut ci = self.comments.partition_point(|c| c.end <= node.span.start);
        while let Some(c) = self.comments.get(ci) {
            if c.start >= node.span.end {
                break;
            }
            let in_child = match &node.children {
                Children::Array(elems) => {
                    let first = elems.partition_point(|e| e.span.end <= c.start);
                    elems.get(first).is_some_and(|e| e.span.contains(&c.start))
                }
                Children::Object(members) => {
                    let first = members.partition_point(|(_, v)| v.span.end <= c.start);
                    members.get(first).is_some_and(|(k, v)| {
                        k.span.contains(&c.start) || v.span.contains(&c.start)
                    })
                }
                Children::Leaf => false,
            };
            if !in_child {
                return true;
            }
            ci += 1;
        }
        false
    }

    // --- Emission helpers --------------------------------------------

    fn indent(&mut self) {
        let n = self.settings.indent_size.saturating_mul(self.level);
        const SPACES: &str = "                                                                ";
        let mut remaining = n;
        while remaining >= SPACES.len() {
            self.out.push_str(SPACES);
            remaining -= SPACES.len();
        }
        if remaining > 0 {
            self.out.push_str(&SPACES[..remaining]);
        }
    }

    /// Ends the current line (unless already at its start), keeping a single
    /// blank line when the input had one between `last_content_end` and
    /// `next_pos`. The caller indents the new line with [`Formatter::indent`],
    /// usually after adjusting the nesting level.
    fn newline_and_blank(&mut self, last_content_end: usize, next_pos: usize) {
        if self.out.column() != 0 {
            self.out.push('\n');
        }
        if self.has_blank_line(last_content_end, next_pos) {
            self.out.push('\n');
        }
    }

    /// Emits the items of a gap that follows `prev_end`. Each item is placed
    /// on its own indented line when `force_own_line` is set, or when it
    /// started on a later line than the previous content in the input;
    /// otherwise it stays on the same line (comments get a leading space,
    /// commas and colons do not). Multiple blank lines collapse to one.
    ///
    /// Returns `(last input line, end of the last emitted item)`.
    fn emit_gap_items(
        &mut self,
        prev_end: usize,
        items: &[Item],
        initial_at_line_start: bool,
        force_own_line: bool,
    ) -> (usize, usize) {
        let mut at_line_start = initial_at_line_start;
        let mut last_line = 0;
        let mut last_content_end = prev_end;
        for item in items {
            let line = if force_own_line {
                last_line + 1
            } else {
                self.line_index(prev_end, item.position())
            };
            if line > last_line {
                if self.out.column() != 0 {
                    self.out.push('\n');
                }
                if self.has_blank_line(last_content_end, item.position()) {
                    self.out.push('\n');
                }
                self.indent();
                at_line_start = true;
            }
            match item {
                Item::Comment(c) => {
                    if !at_line_start {
                        self.out.push(' ');
                    }
                    self.emit_comment(c.clone());
                    last_content_end = c.end;
                }
                Item::Comma(_) => self.out.push(','),
                Item::Colon(_) => self.out.push(':'),
            }
            at_line_start = false;
            last_line = line;
        }
        (last_line, last_content_end)
    }

    fn emit_comment(&mut self, range: Range<usize>) {
        let raw = &self.text[range.clone()];
        if raw.starts_with("//") {
            // A CRLF-terminated line comment includes the `\r` in its range;
            // the formatter always emits LF, so the `\r` is dropped only when
            // it is the CR of a CRLF (followed by `\n`). A lone `\r` inside
            // the comment is comment content and is kept.
            let mut end = raw.len();
            if raw.ends_with('\r') && self.text[range.end..].starts_with('\n') {
                end -= 1;
            }
            self.out.push_str(&raw[..end]);
        } else {
            self.emit_block_comment(raw, range.start);
        }
    }

    /// Emits a block comment, normalizing CRLF to LF and adjusting the leading
    /// spaces of continuation lines by the difference between the comment's
    /// input and output columns (never adding more than the input had to
    /// remove). Tabs and other comment content are preserved verbatim.
    fn emit_block_comment(&mut self, raw: &str, input_pos: usize) {
        let input_col = self.input_column(input_pos);
        let output_col = self.out.column();
        let delta = output_col as isize - input_col as isize;
        let mut lines = raw.split('\n').peekable();
        let mut first = true;
        while let Some(line) = lines.next() {
            // CRLF normalization: the `\r` of a CRLF precedes the `\n` that
            // ended the line, so it is dropped on every line except the last
            // (where it cannot be part of a CRLF). A lone `\r` inside the
            // comment is comment content and is kept.
            let line = if lines.peek().is_some() {
                line.strip_suffix('\r').unwrap_or(line)
            } else {
                line
            };
            if first {
                self.out.push_str(line);
                first = false;
                continue;
            }
            self.out.push('\n');
            let lead = line.len() - line.trim_start_matches(' ').len();
            let adjusted = (lead as isize + delta).max(0) as usize;
            for _ in 0..adjusted {
                self.out.push(' ');
            }
            self.out.push_str(&line[lead..]);
        }
    }

    /// Byte column of `pos` within its input line. A line starts after a `\n`
    /// only: a `\r` inside a comment is comment content, not a line start.
    /// This matches the output column tracking, which is also reset by `\n`
    /// only, keeping the continuation-line adjustment idempotent.
    fn input_column(&self, pos: usize) -> usize {
        let text = self.text.as_bytes();
        let mut start = pos;
        while start > 0 {
            if text[start - 1] == b'\n' {
                break;
            }
            start -= 1;
        }
        pos - start
    }

    /// Builds the tail-gap item list according to the trailing-comma policy.
    /// `multiline` is the container's final line-break decision.
    fn apply_trailing_comma(&self, node: &Node, multiline: bool, mut tail: Vec<Item>) -> Vec<Item> {
        let has_content = match &node.children {
            Children::Array(elems) => !elems.is_empty(),
            Children::Object(members) => !members.is_empty(),
            Children::Leaf => false,
        };
        let has_comma = tail.iter().any(|i| matches!(i, Item::Comma(_)));
        match self.settings.trailing_commas {
            JsoncTrailingCommas::Preserve => tail,
            JsoncTrailingCommas::Never => {
                tail.retain(|i| !matches!(i, Item::Comma(_)));
                tail
            }
            JsoncTrailingCommas::AlwaysMultiline => {
                if multiline && has_content && has_comma {
                    // Keep the existing comma where the input had it.
                    tail
                } else if multiline && has_content {
                    // Insert the comma right after the last element or member,
                    // before any trailing comments.
                    let pos = match &node.children {
                        Children::Array(elems) => elems[elems.len() - 1].span.end,
                        Children::Object(members) => members[members.len() - 1].1.span.end,
                        Children::Leaf => unreachable!(),
                    };
                    let mut out = Vec::with_capacity(tail.len() + 1);
                    out.push(Item::Comma(pos));
                    out.extend(tail);
                    out
                } else {
                    // Single-line containers, empty containers, and
                    // comment-only containers get no trailing comma.
                    tail.retain(|i| !matches!(i, Item::Comma(_)));
                    tail
                }
            }
        }
    }

    // --- Multi-line emission -----------------------------------------

    fn emit_multiline_array(&mut self, node: &Node) {
        let elems = match &node.children {
            Children::Array(elems) => elems,
            _ => unreachable!(),
        };
        let open = node.span.start;
        let close = node.span.end - 1;
        self.out.push('[');
        self.level += 1;

        if elems.is_empty() {
            let head = self.scan_gap(open + 1, close);
            let (_, last_content_end) = self.emit_gap_items(open + 1, &head, false, true);
            self.newline_and_blank(last_content_end, close);
            self.level -= 1;
            self.indent();
            self.out.push(']');
            return;
        }

        for (i, e) in elems.iter().enumerate() {
            let prev_end = if i == 0 {
                open + 1
            } else {
                elems[i - 1].span.end
            };
            let gap = if i == 0 {
                self.scan_gap(open + 1, e.span.start)
            } else {
                self.scan_gap(elems[i - 1].span.end, e.span.start)
            };
            let (_, last_content_end) = self.emit_gap_items(prev_end, &gap, false, i == 0);
            self.newline_and_blank(last_content_end, e.span.start);
            self.indent();
            self.emit_value(e);
        }

        let last_end = elems[elems.len() - 1].span.end;
        let tail = self.apply_trailing_comma(node, true, self.scan_gap(last_end, close));
        let (_, last_content_end) = self.emit_gap_items(last_end, &tail, false, false);
        self.newline_and_blank(last_content_end, close);
        self.level -= 1;
        self.indent();
        self.out.push(']');
    }

    fn emit_multiline_object(&mut self, node: &Node) {
        let members = match &node.children {
            Children::Object(members) => members,
            _ => unreachable!(),
        };
        let open = node.span.start;
        let close = node.span.end - 1;
        self.out.push('{');
        self.level += 1;

        if members.is_empty() {
            let head = self.scan_gap(open + 1, close);
            let (_, last_content_end) = self.emit_gap_items(open + 1, &head, false, true);
            self.newline_and_blank(last_content_end, close);
            self.level -= 1;
            self.indent();
            self.out.push('}');
            return;
        }

        for (i, (k, v)) in members.iter().enumerate() {
            let prev_end = if i == 0 {
                open + 1
            } else {
                members[i - 1].1.span.end
            };
            let gap = if i == 0 {
                self.scan_gap(open + 1, k.span.start)
            } else {
                self.scan_gap(members[i - 1].1.span.end, k.span.start)
            };
            let (_, last_content_end) = self.emit_gap_items(prev_end, &gap, false, i == 0);
            self.newline_and_blank(last_content_end, k.span.start);
            self.indent();
            self.out.push_str(&self.text[k.span.clone()]);

            let key_colon = self.scan_gap(k.span.end, v.span.start);
            let (last_line, last_content_end) =
                self.emit_gap_items(k.span.end, &key_colon, false, false);
            let vline = self.line_index(k.span.end, v.span.start);
            if vline > last_line {
                self.newline_and_blank(last_content_end, v.span.start);
                self.indent();
            } else if self.out.column() != 0 {
                self.out.push(' ');
            }
            self.emit_value(v);
        }

        let last_end = members[members.len() - 1].1.span.end;
        let tail = self.apply_trailing_comma(node, true, self.scan_gap(last_end, close));
        let (_, last_content_end) = self.emit_gap_items(last_end, &tail, false, false);
        self.newline_and_blank(last_content_end, close);
        self.level -= 1;
        self.indent();
        self.out.push('}');
    }

    // --- Single-line emission ----------------------------------------

    fn emit_single_line(&mut self, node: &Node) {
        match &node.children {
            Children::Array(_) => self.emit_single_line_array(node),
            Children::Object(_) => self.emit_single_line_object(node),
            Children::Leaf => unreachable!(),
        }
    }

    fn emit_single_line_array(&mut self, node: &Node) {
        let elems = match &node.children {
            Children::Array(elems) => elems,
            _ => unreachable!(),
        };
        let open = node.span.start;
        let close = node.span.end - 1;
        self.out.push('[');
        if elems.is_empty() {
            let head = self.scan_gap(open + 1, close);
            if head.is_empty() {
                self.out.push(']');
                return;
            }
            self.out.push(' ');
            for item in &head {
                if let Item::Comment(c) = item {
                    self.emit_comment(c.clone());
                    self.out.push(' ');
                }
            }
            self.out.push(']');
            return;
        }

        let head = self.scan_gap(open + 1, elems[0].span.start);
        for item in &head {
            if let Item::Comment(c) = item {
                self.out.push(' ');
                self.emit_comment(c.clone());
            }
        }
        for (i, e) in elems.iter().enumerate() {
            if i > 0 {
                let between = self.scan_gap(elems[i - 1].span.end, e.span.start);
                for item in &between {
                    match item {
                        Item::Comment(c) => {
                            self.out.push(' ');
                            self.emit_comment(c.clone());
                        }
                        Item::Comma(_) => self.out.push(','),
                        Item::Colon(_) => unreachable!(),
                    }
                }
                self.out.push(' ');
            } else if !head.is_empty() {
                self.out.push(' ');
            }
            self.emit_value(e);
        }

        let last_end = elems[elems.len() - 1].span.end;
        let tail = self.apply_trailing_comma(node, false, self.scan_gap(last_end, close));
        for item in &tail {
            match item {
                Item::Comment(c) => {
                    self.out.push(' ');
                    self.emit_comment(c.clone());
                }
                Item::Comma(_) => self.out.push(','),
                Item::Colon(_) => unreachable!(),
            }
        }
        self.out.push(']');
    }

    fn emit_single_line_object(&mut self, node: &Node) {
        let members = match &node.children {
            Children::Object(members) => members,
            _ => unreachable!(),
        };
        let open = node.span.start;
        let close = node.span.end - 1;
        self.out.push('{');
        if members.is_empty() {
            let head = self.scan_gap(open + 1, close);
            if head.is_empty() {
                self.out.push('}');
                return;
            }
            self.out.push(' ');
            for item in &head {
                if let Item::Comment(c) = item {
                    self.emit_comment(c.clone());
                    self.out.push(' ');
                }
            }
            self.out.push('}');
            return;
        }

        let head = self.scan_gap(open + 1, members[0].0.span.start);
        for item in &head {
            if let Item::Comment(c) = item {
                self.out.push(' ');
                self.emit_comment(c.clone());
            }
        }
        for (i, (k, v)) in members.iter().enumerate() {
            if i > 0 {
                let between = self.scan_gap(members[i - 1].1.span.end, k.span.start);
                for item in &between {
                    match item {
                        Item::Comment(c) => {
                            self.out.push(' ');
                            self.emit_comment(c.clone());
                        }
                        Item::Comma(_) => self.out.push(','),
                        Item::Colon(_) => unreachable!(),
                    }
                }
                self.out.push(' ');
            } else if !head.is_empty() {
                self.out.push(' ');
            }
            self.out.push_str(&self.text[k.span.clone()]);
            let key_colon = self.scan_gap(k.span.end, v.span.start);
            for item in &key_colon {
                match item {
                    Item::Comment(c) => {
                        self.out.push(' ');
                        self.emit_comment(c.clone());
                    }
                    Item::Colon(_) => self.out.push(':'),
                    Item::Comma(_) => unreachable!(),
                }
            }
            self.out.push(' ');
            self.emit_value(v);
        }

        let last_end = members[members.len() - 1].1.span.end;
        let tail = self.apply_trailing_comma(node, false, self.scan_gap(last_end, close));
        for item in &tail {
            match item {
                Item::Comment(c) => {
                    self.out.push(' ');
                    self.emit_comment(c.clone());
                }
                Item::Comma(_) => self.out.push(','),
                Item::Colon(_) => unreachable!(),
            }
        }
        self.out.push('}');
    }
}
