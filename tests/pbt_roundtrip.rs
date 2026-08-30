//! Roundtrip property tests for nojson, driven by noprop.
//!
//! Every test samples a value with `noprop::sample_*`, serialises it
//! with `nojson::Json(v).to_string()`, parses the result back, and
//! asserts the parsed value equals the original.
//!
//! The shared harness (runner config and value generators) lives in
//! `pbt_harness`, which `pbt_pretty.rs` draws from as well. Set
//! `NOJSON_PBT_SEED` to a value from a failure report (hex and `_`
//! separators are accepted) to reproduce that run; otherwise a fresh
//! time-derived seed is used.

mod pbt_harness;

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

use pbt_harness::*;

// --- Roundtrip tests -------------------------------------------------

#[test]
fn roundtrip_bool() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_bool(ctx);
        let parsed: nojson::Json<bool> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i8() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i8(ctx);
        let parsed: nojson::Json<i8> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i16() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i16(ctx);
        let parsed: nojson::Json<i16> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i32() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i32(ctx);
        let parsed: nojson::Json<i32> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i64() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i64(ctx);
        let parsed: nojson::Json<i64> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i128() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_i128(ctx);
        let parsed: nojson::Json<i128> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u8() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u8(ctx);
        let parsed: nojson::Json<u8> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u16() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u16(ctx);
        let parsed: nojson::Json<u16> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u32() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u32(ctx);
        let parsed: nojson::Json<u32> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u64() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u64(ctx);
        let parsed: nojson::Json<u64> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u128() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_u128(ctx);
        let parsed: nojson::Json<u128> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_isize() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_isize(ctx);
        let parsed: nojson::Json<isize> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_usize() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_usize(ctx);
        let parsed: nojson::Json<usize> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_f32_finite() -> noprop::TestResult {
    run(|ctx| {
        // `sample_f32` already rejects non-finite draws internally.
        let v = noprop::sample_f32(ctx);
        let parsed: nojson::Json<f32> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_f64_finite() -> noprop::TestResult {
    run(|ctx| {
        let v = noprop::sample_f64(ctx);
        let parsed: nojson::Json<f64> = nojson::Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_string() -> noprop::TestResult {
    let escape_cases = Cell::new(0usize);
    run(|ctx| {
        let s = sample_string_arbitrary(ctx);
        let parsed: nojson::Json<String> = nojson::Json(&s).to_string().parse()?;
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
        let parsed: nojson::Json<String> = nojson::Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

#[test]
fn roundtrip_char() -> noprop::TestResult {
    run(|ctx| {
        let c = sample_char_any(ctx);
        let parsed: nojson::Json<char> = nojson::Json(c).to_string().parse()?;
        assert_eq!(parsed.0, c);
        Ok(())
    })
}

#[test]
fn roundtrip_option_i32() -> noprop::TestResult {
    run(|ctx| {
        let opt = sample_option(ctx, noprop::sample_i32);
        let parsed: nojson::Json<Option<i32>> = nojson::Json(opt).to_string().parse()?;
        assert_eq!(parsed.0, opt);
        Ok(())
    })
}

#[test]
fn roundtrip_option_string() -> noprop::TestResult {
    run(|ctx| {
        let opt = sample_option(ctx, sample_string_arbitrary);
        let parsed: nojson::Json<Option<String>> =
            nojson::Json(opt.as_ref()).to_string().parse()?;
        assert_eq!(parsed.0, opt);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_i32() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, noprop::sample_i32);
        let parsed: nojson::Json<Vec<i32>> = nojson::Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_string() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, sample_string_arbitrary);
        let parsed: nojson::Json<Vec<String>> = nojson::Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_option_i32() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, |ctx| sample_option(ctx, noprop::sample_i32));
        let parsed: nojson::Json<Vec<Option<i32>>> = nojson::Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_nested_vec() -> noprop::TestResult {
    run(|ctx| {
        let v = sample_vec(ctx, |ctx| sample_vec(ctx, noprop::sample_i32));
        let parsed: nojson::Json<Vec<Vec<i32>>> = nojson::Json(&v).to_string().parse()?;
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
        let parsed: nojson::Json<BTreeMap<String, i32>> = nojson::Json(&m).to_string().parse()?;
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
        let parsed: nojson::Json<BTreeMap<String, Option<String>>> =
            nojson::Json(&m).to_string().parse()?;
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
        let parsed: nojson::Json<[i32; 5]> = nojson::Json(arr).to_string().parse()?;
        assert_eq!(parsed.0, arr);
        Ok(())
    })
}

// --- NonZero types ---------------------------------------------------

#[test]
fn roundtrip_nonzero_i8() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i8(ctx);
        let parsed: nojson::Json<NonZeroI8> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u8() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u8(ctx);
        let parsed: nojson::Json<NonZeroU8> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i16() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i16(ctx);
        let parsed: nojson::Json<NonZeroI16> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u16() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u16(ctx);
        let parsed: nojson::Json<NonZeroU16> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i32() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i32(ctx);
        let parsed: nojson::Json<NonZeroI32> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u32() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u32(ctx);
        let parsed: nojson::Json<NonZeroU32> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i64() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i64(ctx);
        let parsed: nojson::Json<NonZeroI64> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u64() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u64(ctx);
        let parsed: nojson::Json<NonZeroU64> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i128() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_i128(ctx);
        let parsed: nojson::Json<NonZeroI128> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u128() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_u128(ctx);
        let parsed: nojson::Json<NonZeroU128> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_isize() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_isize(ctx);
        let parsed: nojson::Json<NonZeroIsize> = nojson::Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_usize() -> noprop::TestResult {
    run(|ctx| {
        let nz = sample_non_zero_usize(ctx);
        let parsed: nojson::Json<NonZeroUsize> = nojson::Json(nz).to_string().parse()?;
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
        let parsed: nojson::Json<i32> = nojson::Json(&b).to_string().parse()?;
        assert_eq!(parsed.0, n);
        Ok(())
    })
}

#[test]
fn roundtrip_rc_string() -> noprop::TestResult {
    run(|ctx| {
        let s = sample_string_arbitrary(ctx);
        let r = Rc::new(s.clone());
        let parsed: nojson::Json<Rc<String>> = nojson::Json(&r).to_string().parse()?;
        assert_eq!(parsed.0.as_ref(), &s);
        Ok(())
    })
}

#[test]
fn roundtrip_arc_string() -> noprop::TestResult {
    run(|ctx| {
        let s = sample_string_arbitrary(ctx);
        let a = Arc::new(s.clone());
        let parsed: nojson::Json<Arc<String>> = nojson::Json(&a).to_string().parse()?;
        assert_eq!(parsed.0.as_ref(), &s);
        Ok(())
    })
}

// --- Additional collections ------------------------------------------

#[test]
fn roundtrip_vecdeque_i32() -> noprop::TestResult {
    run(|ctx| {
        let v: VecDeque<i32> = sample_vec(ctx, noprop::sample_i32).into();
        let parsed: nojson::Json<VecDeque<i32>> = nojson::Json(&v).to_string().parse()?;
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
        let parsed: nojson::Json<BTreeSet<i32>> = nojson::Json(&s).to_string().parse()?;
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
        let parsed: nojson::Json<HashMap<String, i32>> = nojson::Json(&m).to_string().parse()?;
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
        let parsed: nojson::Json<HashSet<i32>> = nojson::Json(&s).to_string().parse()?;
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
        let parsed: nojson::Json<PathBuf> = nojson::Json(&p).to_string().parse()?;
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
        let parsed: nojson::Json<Ipv4Addr> = nojson::Json(ip).to_string().parse()?;
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
        let parsed: nojson::Json<Ipv6Addr> = nojson::Json(ip).to_string().parse()?;
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
        let parsed: nojson::Json<IpAddr> = nojson::Json(ip).to_string().parse()?;
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
        let parsed: nojson::Json<SocketAddrV4> = nojson::Json(addr).to_string().parse()?;
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
        let parsed: nojson::Json<SocketAddrV6> = nojson::Json(addr).to_string().parse()?;
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
        let parsed: nojson::Json<SocketAddr> = nojson::Json(addr).to_string().parse()?;
        assert_eq!(parsed.0, addr);
        Ok(())
    })
}

// --- Unit ------------------------------------------------------------

#[test]
fn roundtrip_unit() -> noprop::TestResult {
    run(|_ctx| {
        let parsed: nojson::Json<()> = nojson::Json(()).to_string().parse()?;
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
        let parsed: nojson::Json<Vec<BTreeMap<String, i32>>> =
            nojson::Json(&v).to_string().parse()?;
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
        let parsed: nojson::Json<BTreeMap<String, Vec<i32>>> =
            nojson::Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}
