//! Compact storage for parsed JSON value entries.
//!
//! Each entry holds a text range and structural link. Positions are stored as
//! `u32` (capping single documents at `u32::MAX` bytes — the parser refuses
//! larger inputs), and the kind + escaped flag share one byte, so an entry
//! costs 16 bytes instead of the 32 that `usize` fields required. Halving the
//! per-entry footprint doubles the number of entries that fit in a cache line
//! during structural traversal.

use alloc::vec::Vec;
use core::ops::Range;

use crate::JsonValueKind;

const KIND_MASK: u8 = 0x7F;
const ESCAPED_BIT: u8 = 0x80;

/// One parsed JSON value slot. 16 bytes on all targets (asserted below).
#[derive(Debug, Clone, Copy)]
pub(crate) struct JsonValueIndexEntry {
    // Text range in the source, `[text_start, text_end)`.
    text_start: u32,
    text_end: u32,
    // For containers, one past the last descendant slot; for scalars, `index + 1`.
    end_index: u32,
    // Bit 7 = escaped flag; bits 0-6 = kind ordinal.
    packed: u8,
}

const _: () = assert!(core::mem::size_of::<JsonValueIndexEntry>() == 16);

impl JsonValueIndexEntry {
    pub(crate) fn new(kind: JsonValueKind, start: u32, end: u32, end_index: u32) -> Self {
        Self {
            text_start: start,
            text_end: end,
            end_index,
            packed: kind_to_byte(kind),
        }
    }

    pub(crate) fn kind(&self) -> JsonValueKind {
        byte_to_kind(self.packed & KIND_MASK)
    }

    pub(crate) fn escaped(&self) -> bool {
        self.packed & ESCAPED_BIT != 0
    }

    pub(crate) fn text_start(&self) -> usize {
        self.text_start as usize
    }

    pub(crate) fn text_end(&self) -> usize {
        self.text_end as usize
    }

    pub(crate) fn text_range(&self) -> Range<usize> {
        self.text_start()..self.text_end()
    }

    pub(crate) fn end_index(&self) -> usize {
        self.end_index as usize
    }

    pub(crate) fn set_text_end(&mut self, end: u32) {
        self.text_end = end;
    }

    pub(crate) fn set_end_index(&mut self, end_index: u32) {
        self.end_index = end_index;
    }

    pub(crate) fn set_escaped(&mut self, escaped: bool) {
        self.packed = (self.packed & KIND_MASK) | if escaped { ESCAPED_BIT } else { 0 };
    }

    /// Adjust text positions and end_index by the given offsets. Used by
    /// [`RawJsonValue::extract`] when materialising a sub-tree.
    pub(crate) fn shifted(&self, text_offset: u32, index_offset: u32) -> Self {
        Self {
            text_start: self.text_start - text_offset,
            text_end: self.text_end - text_offset,
            end_index: self.end_index - index_offset,
            packed: self.packed,
        }
    }
}

/// Owning column of entries. Kept as a thin newtype so future refactors (say,
/// splitting into SoA if profiling later warrants it) can happen behind this
/// crate-internal API without touching call sites.
#[derive(Debug, Clone, Default)]
pub(crate) struct JsonValues(Vec<JsonValueIndexEntry>);

impl JsonValues {
    pub(crate) fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn push(&mut self, entry: JsonValueIndexEntry) {
        self.0.push(entry);
    }

    pub(crate) fn get(&self, i: usize) -> &JsonValueIndexEntry {
        &self.0[i]
    }

    pub(crate) fn get_mut(&mut self, i: usize) -> &mut JsonValueIndexEntry {
        &mut self.0[i]
    }

    pub(crate) fn slice(&self, range: Range<usize>) -> &[JsonValueIndexEntry] {
        &self.0[range]
    }

    pub(crate) fn from_iter<I: IntoIterator<Item = JsonValueIndexEntry>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[inline(always)]
fn kind_to_byte(kind: JsonValueKind) -> u8 {
    match kind {
        JsonValueKind::Null => 0,
        JsonValueKind::Boolean => 1,
        JsonValueKind::Integer => 2,
        JsonValueKind::Float => 3,
        JsonValueKind::String => 4,
        JsonValueKind::Array => 5,
        JsonValueKind::Object => 6,
    }
}

#[inline(always)]
fn byte_to_kind(byte: u8) -> JsonValueKind {
    match byte {
        0 => JsonValueKind::Null,
        1 => JsonValueKind::Boolean,
        2 => JsonValueKind::Integer,
        3 => JsonValueKind::Float,
        4 => JsonValueKind::String,
        5 => JsonValueKind::Array,
        6 => JsonValueKind::Object,
        _ => unreachable!("packed byte was written by kind_to_byte"),
    }
}
