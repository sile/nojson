//! Roundtrip property tests for nojson, driven by noprop.
//!
//! Every test samples a value with `noprop::sample_*`, serialises it
//! with `nojson::Json(v).to_string()`, parses the result back, and
//! asserts the parsed value equals the original.
//!
//! All noprop primitives are qualified with the full crate path (no
//! `use noprop::*` shortcuts) so it is immediately obvious which
//! primitive each call reaches for.
//!
//! Set `NOJSON_PBT_SEED` to a value from a failure report (hex and
//! `_` separators are accepted) to reproduce that run; otherwise a
//! fresh time-derived seed is used.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "std")]
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};
#[cfg(feature = "std")]
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use nojson::Json;
use noprop::TestCaseContext;

// --- Runner config ---------------------------------------------------

/// Per-test case budget.
const CASES: usize = 256;

/// Upper bound (inclusive) on generated collection / string lengths.
/// Kept small so nested collections don't blow up. Must be at least 2
/// (see `sample_len`).
const MAX_LEN: usize = 8;

/// Run `f` under the case budget, with the seed taken from
/// `NOJSON_PBT_SEED` when set (reproducing a failure report) and a
/// fresh time-derived seed otherwise. Every test in this file uses
/// the same runner shape.
fn run<F>(f: F) -> noprop::TestResult
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
fn sample_len(ctx: &mut TestCaseContext, max: usize) -> usize {
    debug_assert!(max >= 2, "sample_len requires max >= 2");
    noprop::sample_with_boundaries(ctx, &[0, max], noprop::Ratio::one_nth(5), |ctx| {
        noprop::sample_usize_in(ctx, 1..max)
    })
}

fn sample_option<T>(
    ctx: &mut TestCaseContext,
    f: impl FnOnce(&mut TestCaseContext) -> T,
) -> Option<T> {
    if noprop::sample_bool(ctx) {
        Some(f(ctx))
    } else {
        None
    }
}

fn sample_vec<T>(
    ctx: &mut TestCaseContext,
    mut f: impl FnMut(&mut TestCaseContext) -> T,
) -> Vec<T> {
    let n = sample_len(ctx, MAX_LEN);
    (0..n).map(|_| f(ctx)).collect()
}

/// `true` for the characters the JSON formatter must escape: `"`, `\`,
/// and the ASCII control characters (`0x00..=0x1F`).
fn needs_escape(c: char) -> bool {
    matches!(c, '"' | '\\') || (c as u32) < 0x20
}

/// Sample a character that forces JSON escaping, covering every
/// escape form the formatter emits: `\"`, `\\`, `\n`, `\r`, `\t`,
/// `\b`, `\f`, and the `\uXXXX` path.
fn sample_escape_char(ctx: &mut TestCaseContext) -> char {
    noprop::sample_choice(
        ctx,
        &['"', '\\', '\n', '\r', '\t', '\u{8}', '\u{c}', '\u{0}'],
    )
}

/// Sample a character from the full Unicode domain, or an
/// escape-forcing character with equal probability — the escape path
/// is exercised routinely instead of with negligible probability.
fn sample_char_any(ctx: &mut TestCaseContext) -> char {
    if noprop::sample_ratio(ctx, noprop::Ratio::one_nth(2)) {
        noprop::sample_char(ctx)
    } else {
        sample_escape_char(ctx)
    }
}

/// Sample a string whose length comes from `sample_len` and whose
/// characters come from `sample_char_any`.
fn sample_string_arbitrary(ctx: &mut TestCaseContext) -> String {
    let n = sample_len(ctx, MAX_LEN);
    (0..n).map(|_| sample_char_any(ctx)).collect()
}

/// ASCII printable, excluding `"` and `\` — mirrors the ASCII
/// character set used by the original proptest `plain_ascii_string`
/// helper.
fn sample_string_ascii_plain(ctx: &mut TestCaseContext, min: usize, max: usize) -> String {
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
fn sample_string_mixed(ctx: &mut TestCaseContext) -> String {
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
fn sample_non_zero_i8(ctx: &mut TestCaseContext) -> NonZeroI8 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI8::new(noprop::sample_i8(ctx)))
}

#[track_caller]
fn sample_non_zero_u8(ctx: &mut TestCaseContext) -> NonZeroU8 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU8::new(noprop::sample_u8(ctx)))
}

#[track_caller]
fn sample_non_zero_i16(ctx: &mut TestCaseContext) -> NonZeroI16 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI16::new(noprop::sample_i16(ctx)))
}

#[track_caller]
fn sample_non_zero_u16(ctx: &mut TestCaseContext) -> NonZeroU16 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU16::new(noprop::sample_u16(ctx)))
}

#[track_caller]
fn sample_non_zero_i32(ctx: &mut TestCaseContext) -> NonZeroI32 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI32::new(noprop::sample_i32(ctx)))
}

#[track_caller]
fn sample_non_zero_u32(ctx: &mut TestCaseContext) -> NonZeroU32 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU32::new(noprop::sample_u32(ctx)))
}

#[track_caller]
fn sample_non_zero_i64(ctx: &mut TestCaseContext) -> NonZeroI64 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI64::new(noprop::sample_i64(ctx)))
}

#[track_caller]
fn sample_non_zero_u64(ctx: &mut TestCaseContext) -> NonZeroU64 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU64::new(noprop::sample_u64(ctx)))
}

#[track_caller]
fn sample_non_zero_i128(ctx: &mut TestCaseContext) -> NonZeroI128 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroI128::new(noprop::sample_i128(ctx)))
}

#[track_caller]
fn sample_non_zero_u128(ctx: &mut TestCaseContext) -> NonZeroU128 {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroU128::new(noprop::sample_u128(ctx)))
}

#[track_caller]
fn sample_non_zero_isize(ctx: &mut TestCaseContext) -> NonZeroIsize {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroIsize::new(noprop::sample_isize(ctx)))
}

#[track_caller]
fn sample_non_zero_usize(ctx: &mut TestCaseContext) -> NonZeroUsize {
    noprop::sample_with_rejection(ctx, 64, |ctx| NonZeroUsize::new(noprop::sample_usize(ctx)))
}

// --- Roundtrip tests -------------------------------------------------

#[test]
fn roundtrip_bool() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_bool(ctx);
        let parsed: Json<bool> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i8() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i8(ctx);
        let parsed: Json<i8> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i16() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i16(ctx);
        let parsed: Json<i16> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i32() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i32(ctx);
        let parsed: Json<i32> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i64() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i64(ctx);
        let parsed: Json<i64> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i128() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i128(ctx);
        let parsed: Json<i128> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u8() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u8(ctx);
        let parsed: Json<u8> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u16() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u16(ctx);
        let parsed: Json<u16> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u32() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u32(ctx);
        let parsed: Json<u32> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u64() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u64(ctx);
        let parsed: Json<u64> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u128() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u128(ctx);
        let parsed: Json<u128> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_isize() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_isize(ctx);
        let parsed: Json<isize> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_usize() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_usize(ctx);
        let parsed: Json<usize> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_f32_finite() -> noprop::TestResult {
    run(|ctx| {
        // `sample_f32` already rejects non-finite draws internally.
        let v = noprop::sample_f32(ctx);
        let parsed: Json<f32> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_f64_finite() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_f64(ctx);
        let parsed: Json<f64> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_string() -> noprop::TestResult {
    let escape_cases = Cell::new(0usize);
    run(|ctx| {
        let s = sample_string_arbitrary(ctx);
        let parsed: Json<String> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        if s.chars().any(needs_escape) {
            escape_cases.set(escape_cases.get() + 1);
        }
        Ok(())
    })?;
    assert!(
        escape_cases.get() > 0,
        "no case exercised a string that requires JSON escaping"
    );
    Ok(())
}

#[test]
fn roundtrip_string_with_non_ascii_followed_by_ascii() -> noprop::TestResult {
    run(|ctx| {
        let s = sample_string_mixed(ctx);
        let parsed: Json<String> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

#[test]
fn roundtrip_char() -> noprop::TestResult {
    run(|ctx| {
        let c = sample_char_any(ctx);
        let parsed: Json<char> = Json(c).to_string().parse()?;
        assert_eq!(parsed.0, c);
        Ok(())
    })
}

#[test]
fn roundtrip_option_i32() -> noprop::TestResult {
    run(|ctx| {
        let opt = sample_option(ctx, noprop::sample_i32);
        let parsed: Json<Option<i32>> = Json(opt).to_string().parse()?;
        assert_eq!(parsed.0, opt);
        Ok(())
    })
}

#[test]
fn roundtrip_option_string() -> noprop::TestResult {
    run(|ctx| {
        let opt = sample_option(ctx, sample_string_arbitrary);
        let parsed: Json<Option<String>> = Json(opt.as_ref()).to_string().parse()?;
        assert_eq!(parsed.0, opt);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_i32() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, noprop::sample_i32);
        let parsed: Json<Vec<i32>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_string() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, sample_string_arbitrary);
        let parsed: Json<Vec<String>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_option_i32() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, |ctx| sample_option(ctx, noprop::sample_i32));
        let parsed: Json<Vec<Option<i32>>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_nested_vec() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, |ctx| sample_vec(ctx, noprop::sample_i32));
        let parsed: Json<Vec<Vec<i32>>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_btreemap_string_i32() -> noprop::TestResult {
    run(|ctx| {
        let n = sample_len(ctx, MAX_LEN);
        let mut m = BTreeMap::new();
        for _ in 0..n {
            m.insert(sample_string_arbitrary(ctx), noprop::sample_i32(ctx));
        }
        let parsed: Json<BTreeMap<String, i32>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}

#[test]
fn roundtrip_btreemap_string_option_string() -> noprop::TestResult {
    run(|ctx| {
        let n = sample_len(ctx, MAX_LEN);
        let mut m = BTreeMap::new();
        for _ in 0..n {
            let k = sample_string_arbitrary(ctx);
            let v = sample_option(ctx, sample_string_arbitrary);
            m.insert(k, v);
        }
        let parsed: Json<BTreeMap<String, Option<String>>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}

#[test]
fn roundtrip_array_fixed() -> noprop::TestResult {
    run(|ctx| {
        let arr: [i32; 5] = [
            noprop::sample_i32(ctx),
            noprop::sample_i32(ctx),
            noprop::sample_i32(ctx),
            noprop::sample_i32(ctx),
            noprop::sample_i32(ctx),
        ];
        let parsed: Json<[i32; 5]> = Json(arr).to_string().parse()?;
        assert_eq!(parsed.0, arr);
        Ok(())
    })
}

// --- NonZero types ---------------------------------------------------

#[test]
fn roundtrip_nonzero_i8() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i8(ctx);
        let parsed: Json<NonZeroI8> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u8() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u8(ctx);
        let parsed: Json<NonZeroU8> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i16() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i16(ctx);
        let parsed: Json<NonZeroI16> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u16() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u16(ctx);
        let parsed: Json<NonZeroU16> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i32() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i32(ctx);
        let parsed: Json<NonZeroI32> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u32() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u32(ctx);
        let parsed: Json<NonZeroU32> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i64() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i64(ctx);
        let parsed: Json<NonZeroI64> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u64() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u64(ctx);
        let parsed: Json<NonZeroU64> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i128() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i128(ctx);
        let parsed: Json<NonZeroI128> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u128() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u128(ctx);
        let parsed: Json<NonZeroU128> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_isize() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_isize(ctx);
        let parsed: Json<NonZeroIsize> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_usize() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_usize(ctx);
        let parsed: Json<NonZeroUsize> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

// --- Smart pointers --------------------------------------------------

#[test]
fn roundtrip_box_i32() -> noprop::TestResult {
    run(|ctx| {
        let n = noprop::sample_i32(ctx);
        let b = Box::new(n);
        let parsed: Json<i32> = Json(&b).to_string().parse()?;
        assert_eq!(parsed.0, n);
        Ok(())
    })
}

#[test]
fn roundtrip_rc_string() -> noprop::TestResult {
    run(|ctx| {
        let s = sample_string_arbitrary(ctx);
        let r = Rc::new(s.clone());
        let parsed: Json<Rc<String>> = Json(&r).to_string().parse()?;
        assert_eq!(parsed.0.as_ref(), &s);
        Ok(())
    })
}

#[test]
fn roundtrip_arc_string() -> noprop::TestResult {
    run(|ctx| {
        let s = sample_string_arbitrary(ctx);
        let a = Arc::new(s.clone());
        let parsed: Json<Arc<String>> = Json(&a).to_string().parse()?;
        assert_eq!(parsed.0.as_ref(), &s);
        Ok(())
    })
}

// --- Additional collections ------------------------------------------

#[test]
fn roundtrip_vecdeque_i32() -> noprop::TestResult {
    run(|ctx| {
        let v: VecDeque<i32> = sample_vec(ctx, noprop::sample_i32).into();
        let parsed: Json<VecDeque<i32>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_btreeset_i32() -> noprop::TestResult {
    run(|ctx| {
        let n = sample_len(ctx, MAX_LEN);
        let mut s = BTreeSet::new();
        for _ in 0..n {
            s.insert(noprop::sample_i32(ctx));
        }
        let parsed: Json<BTreeSet<i32>> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

// --- std-feature types -----------------------------------------------

#[cfg(feature = "std")]
#[test]
fn roundtrip_hashmap_string_i32() -> noprop::TestResult {
    run(|ctx| {
        let n = sample_len(ctx, MAX_LEN);
        let mut m = HashMap::new();
        for _ in 0..n {
            m.insert(sample_string_arbitrary(ctx), noprop::sample_i32(ctx));
        }
        let parsed: Json<HashMap<String, i32>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}

#[cfg(feature = "std")]
#[test]
fn roundtrip_hashset_i32() -> noprop::TestResult {
    run(|ctx| {
        let n = sample_len(ctx, MAX_LEN);
        let mut s = HashSet::new();
        for _ in 0..n {
            s.insert(noprop::sample_i32(ctx));
        }
        let parsed: Json<HashSet<i32>> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

#[cfg(feature = "std")]
#[test]
fn roundtrip_pathbuf() -> noprop::TestResult {
    run(|ctx| {
        let s = sample_string_arbitrary(ctx);
        let p = PathBuf::from(&s);
        let parsed: Json<PathBuf> = Json(&p).to_string().parse()?;
        assert_eq!(parsed.0, p);
        Ok(())
    })
}

// --- Network types ---------------------------------------------------

#[cfg(feature = "std")]
#[test]
fn roundtrip_ipv4addr() -> noprop::TestResult {
    run(|ctx| {
        let ip = Ipv4Addr::new(
            noprop::sample_u8(ctx),
            noprop::sample_u8(ctx),
            noprop::sample_u8(ctx),
            noprop::sample_u8(ctx),
        );
        let parsed: Json<Ipv4Addr> = Json(ip).to_string().parse()?;
        assert_eq!(parsed.0, ip);
        Ok(())
    })
}

#[cfg(feature = "std")]
#[test]
fn roundtrip_ipv6addr() -> noprop::TestResult {
    run(|ctx| {
        let ip = Ipv6Addr::new(
            noprop::sample_u16(ctx),
            noprop::sample_u16(ctx),
            noprop::sample_u16(ctx),
            noprop::sample_u16(ctx),
            noprop::sample_u16(ctx),
            noprop::sample_u16(ctx),
            noprop::sample_u16(ctx),
            noprop::sample_u16(ctx),
        );
        let parsed: Json<Ipv6Addr> = Json(ip).to_string().parse()?;
        assert_eq!(parsed.0, ip);
        Ok(())
    })
}

#[cfg(feature = "std")]
#[test]
fn roundtrip_ipaddr_v4() -> noprop::TestResult {
    run(|ctx| {
        let ip = IpAddr::V4(Ipv4Addr::new(
            noprop::sample_u8(ctx),
            noprop::sample_u8(ctx),
            noprop::sample_u8(ctx),
            noprop::sample_u8(ctx),
        ));
        let parsed: Json<IpAddr> = Json(ip).to_string().parse()?;
        assert_eq!(parsed.0, ip);
        Ok(())
    })
}

#[cfg(feature = "std")]
#[test]
fn roundtrip_socketaddr_v4() -> noprop::TestResult {
    run(|ctx| {
        let addr = SocketAddrV4::new(
            Ipv4Addr::new(
                noprop::sample_u8(ctx),
                noprop::sample_u8(ctx),
                noprop::sample_u8(ctx),
                noprop::sample_u8(ctx),
            ),
            noprop::sample_u16(ctx),
        );
        let parsed: Json<SocketAddrV4> = Json(addr).to_string().parse()?;
        assert_eq!(parsed.0, addr);
        Ok(())
    })
}

#[cfg(feature = "std")]
#[test]
fn roundtrip_socketaddr_v6() -> noprop::TestResult {
    run(|ctx| {
        let addr = SocketAddrV6::new(
            Ipv6Addr::new(
                noprop::sample_u16(ctx),
                noprop::sample_u16(ctx),
                noprop::sample_u16(ctx),
                noprop::sample_u16(ctx),
                noprop::sample_u16(ctx),
                noprop::sample_u16(ctx),
                noprop::sample_u16(ctx),
                noprop::sample_u16(ctx),
            ),
            noprop::sample_u16(ctx),
            0,
            0,
        );
        let parsed: Json<SocketAddrV6> = Json(addr).to_string().parse()?;
        assert_eq!(parsed.0, addr);
        Ok(())
    })
}

#[cfg(feature = "std")]
#[test]
fn roundtrip_socketaddr() -> noprop::TestResult {
    run(|ctx| {
        let addr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(
                noprop::sample_u8(ctx),
                noprop::sample_u8(ctx),
                noprop::sample_u8(ctx),
                noprop::sample_u8(ctx),
            ),
            noprop::sample_u16(ctx),
        ));
        let parsed: Json<SocketAddr> = Json(addr).to_string().parse()?;
        assert_eq!(parsed.0, addr);
        Ok(())
    })
}

// --- Unit ------------------------------------------------------------

#[test]
fn roundtrip_unit() -> noprop::TestResult {
    run(|_ctx| {
        let parsed: Json<()> = Json(()).to_string().parse()?;
        assert_eq!(parsed.0, ());
        Ok(())
    })
}

// --- Deeply nested ---------------------------------------------------

#[test]
fn roundtrip_vec_btreemap() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, |ctx| {
            let n = sample_len(ctx, MAX_LEN);
            let mut m = BTreeMap::new();
            for _ in 0..n {
                m.insert(sample_string_arbitrary(ctx), noprop::sample_i32(ctx));
            }
            m
        });
        let parsed: Json<Vec<BTreeMap<String, i32>>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_btreemap_vec() -> noprop::TestResult {
    run(|ctx| {
        let n = sample_len(ctx, MAX_LEN);
        let mut m = BTreeMap::new();
        for _ in 0..n {
            let k = sample_string_arbitrary(ctx);
            let v = sample_vec(ctx, noprop::sample_i32);
            m.insert(k, v);
        }
        let parsed: Json<BTreeMap<String, Vec<i32>>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}
