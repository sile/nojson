//! Shared harness for the noprop-driven PBT suites.
//!
//! `pbt_roundtrip.rs` (compact output) and `pbt_pretty.rs` (pretty
//! output) both use this module: the runner config (`run`, `CASES`,
//! `MAX_LEN`) and the value generators. The search-space decisions
//! live here in one place so both suites stay in sync:
//!
//! - lengths hit the boundaries `0` and `MAX_LEN` with probability
//!   1/10 each (`sample_len`)
//! - characters force JSON escaping with probability 1/2
//!   (`sample_char_any`)
//! - non-zero integers use uniform bounded rejection
//!   (`sample_non_zero_*`)
//!
//! All noprop primitives are qualified with the full crate path (no
//! `use noprop::*` shortcuts) so it is immediately obvious which
//! primitive each call reaches for.

// Each test crate compiles this module independently; helpers that
// only one suite uses would otherwise warn as dead code.
#![allow(dead_code)]

use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};

use noprop::TestCaseContext;

// --- Runner config ---------------------------------------------------

/// Per-test case budget.
pub const CASES: usize = 256;

/// Upper bound (inclusive) on generated collection / string lengths.
/// Kept small so nested collections don't blow up. Must be at least 2
/// (see `sample_len`).
pub const MAX_LEN: usize = 8;

/// Run `f` under the case budget, with the seed taken from
/// `NOJSON_PBT_SEED` when set (reproducing a failure report) and a
/// fresh time-derived seed otherwise. Every test in the PBT suites
/// uses the same runner shape.
pub fn run<F>(f: F) -> noprop::TestResult
where
    F: Fn(&mut TestCaseContext) -> noprop::TestResult,
{
    let seed = noprop::seed_from_env_or_time("NOJSON_PBT_SEED")?;
    noprop::Runner::new(seed).run(CASES, f)?;
    Ok(())
}

// --- Composite generators --------------------------------------------

/// Sample a length that hits the boundary values `0` and `max` with
/// probability 1/10 each (the boundary set is entered at 1/5) and is
/// uniform in `1..max` otherwise. Empty and maximum-size collections
/// are the two ends of the interesting domain, so they get more than
/// their uniform share. Requires `max >= 2`.
pub fn sample_len(ctx: &mut TestCaseContext, max: usize) -> usize {
    debug_assert!(max >= 2, "sample_len requires max >= 2");
    noprop::sample_with_boundaries(ctx, &[0, max], noprop::Ratio::one_nth(5), |ctx| {
        noprop::sample_usize_in(ctx, 1..max)
    })
}

pub fn sample_option<T>(
    ctx: &mut TestCaseContext,
    f: impl FnOnce(&mut TestCaseContext) -> T,
) -> Option<T> {
    if noprop::sample_bool(ctx) {
        Some(f(ctx))
    } else {
        None
    }
}

pub fn sample_vec<T>(
    ctx: &mut TestCaseContext,
    mut f: impl FnMut(&mut TestCaseContext) -> T,
) -> Vec<T> {
    let n = sample_len(ctx, MAX_LEN);
    (0..n).map(|_| f(ctx)).collect()
}

/// `true` for the characters the JSON formatter must escape: `"`, `\`,
/// and the ASCII control characters (`0x00..=0x1F`).
pub fn needs_escape(c: char) -> bool {
    matches!(c, '"' | '\\') || (c as u32) < 0x20
}

/// Sample a character that forces JSON escaping, covering every
/// escape form the formatter emits: `\"`, `\\`, `\n`, `\r`, `\t`,
/// `\b`, `\f`, and the `\uXXXX` path.
pub fn sample_escape_char(ctx: &mut TestCaseContext) -> char {
    noprop::sample_choice(
        ctx,
        &['"', '\\', '\n', '\r', '\t', '\u{8}', '\u{c}', '\u{0}'],
    )
}

/// Sample a character from the full Unicode domain, or an
/// escape-forcing character with equal probability — the escape path
/// is exercised routinely instead of with negligible probability.
pub fn sample_char_any(ctx: &mut TestCaseContext) -> char {
    if noprop::sample_ratio(ctx, noprop::Ratio::one_nth(2)) {
        noprop::sample_char(ctx)
    } else {
        sample_escape_char(ctx)
    }
}

/// Sample a string whose length comes from `sample_len` and whose
/// characters come from `sample_char_any`.
pub fn sample_string_arbitrary(ctx: &mut TestCaseContext) -> String {
    let n = sample_len(ctx, MAX_LEN);
    (0..n).map(|_| sample_char_any(ctx)).collect()
}

/// ASCII printable, excluding `"` and `\` — mirrors the ASCII
/// character set used by the original proptest `plain_ascii_string`
/// helper.
pub fn sample_string_ascii_plain(ctx: &mut TestCaseContext, min: usize, max: usize) -> String {
    let n = noprop::sample_usize_in(ctx, min..=max);
    (0..n)
        .map(|_| {
            noprop::sample_with_rejection(ctx, 64, |ctx| {
                let c = noprop::sample_ascii_printable_char(ctx);
                (c != '"' && c != '\\').then_some(c)
            })
        })
        .collect()
}

/// Analogue of proptest's `mixed_unicode_ascii_string`: any prefix, a
/// guaranteed non-ASCII char, an ASCII run, then any suffix.
pub fn sample_string_mixed(ctx: &mut TestCaseContext) -> String {
    let mut s = sample_string_arbitrary(ctx);
    let non_ascii = noprop::sample_with_rejection(ctx, 64, |ctx| {
        let c = noprop::sample_char(ctx);
        (!c.is_ascii()).then_some(c)
    });
    s.push(non_ascii);
    s.push_str(&sample_string_ascii_plain(ctx, 1, MAX_LEN));
    s.push_str(&sample_string_arbitrary(ctx));
    s
}

// --- NonZero helpers (uniform via bounded rejection) -----------------
//
// noprop deliberately does not ship `sample_non_zero_*` primitives
// (see the "Sampling non-zero integers" section of its generator
// module docs). These helpers apply the uniform bounded-rejection
// recipe from that section.

#[track_caller]
pub fn sample_non_zero_i8(ctx: &mut TestCaseContext) -> NonZeroI8 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI8::new(noprop::sample_i8(ctx)))
}

#[track_caller]
pub fn sample_non_zero_u8(ctx: &mut TestCaseContext) -> NonZeroU8 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU8::new(noprop::sample_u8(ctx)))
}

#[track_caller]
pub fn sample_non_zero_i16(ctx: &mut TestCaseContext) -> NonZeroI16 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI16::new(noprop::sample_i16(ctx)))
}

#[track_caller]
pub fn sample_non_zero_u16(ctx: &mut TestCaseContext) -> NonZeroU16 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU16::new(noprop::sample_u16(ctx)))
}

#[track_caller]
pub fn sample_non_zero_i32(ctx: &mut TestCaseContext) -> NonZeroI32 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI32::new(noprop::sample_i32(ctx)))
}

#[track_caller]
pub fn sample_non_zero_u32(ctx: &mut TestCaseContext) -> NonZeroU32 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU32::new(noprop::sample_u32(ctx)))
}

#[track_caller]
pub fn sample_non_zero_i64(ctx: &mut TestCaseContext) -> NonZeroI64 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI64::new(noprop::sample_i64(ctx)))
}

#[track_caller]
pub fn sample_non_zero_u64(ctx: &mut TestCaseContext) -> NonZeroU64 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU64::new(noprop::sample_u64(ctx)))
}

#[track_caller]
pub fn sample_non_zero_i128(ctx: &mut TestCaseContext) -> NonZeroI128 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI128::new(noprop::sample_i128(ctx)))
}

#[track_caller]
pub fn sample_non_zero_u128(ctx: &mut TestCaseContext) -> NonZeroU128 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU128::new(noprop::sample_u128(ctx)))
}

#[track_caller]
pub fn sample_non_zero_isize(ctx: &mut TestCaseContext) -> NonZeroIsize {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroIsize::new(noprop::sample_isize(ctx)))
}

#[track_caller]
pub fn sample_non_zero_usize(ctx: &mut TestCaseContext) -> NonZeroUsize {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroUsize::new(noprop::sample_usize(ctx)))
}
