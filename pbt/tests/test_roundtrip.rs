//! Roundtrip property tests for nojson, driven by noprop.
//!
//! Every test picks a value with `noprop::gen_*`, serialises it with
//! `nojson::Json(v).to_string()`, parses the result back, and asserts
//! the parsed value equals the original.
//!
//! All values are qualified with the full crate path (no
//! `use noprop::*` shortcuts) so it is immediately obvious which
//! primitive each call reaches for.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use nojson::Json;

// --- Runner config ---------------------------------------------------

const SEED: u64 = 0xDEAD_BEEF_1234_5678;
const ITERATIONS: usize = 256;

/// Upper bound (inclusive) on generated collection / string lengths.
/// Kept small so nested collections don't blow up.
const MAX_LEN: usize = 8;

fn runner() -> noprop::Runner {
    noprop::Runner {
        seed: SEED,
        iterations: ITERATIONS,
    }
}

// --- Composite generators --------------------------------------------

fn gen_len(rng: &mut noprop::Rng, max: usize) -> usize {
    noprop::gen_usize(rng) % (max + 1)
}

fn gen_option<T>(rng: &mut noprop::Rng, f: impl FnOnce(&mut noprop::Rng) -> T) -> Option<T> {
    if noprop::gen_bool(rng) {
        Some(f(rng))
    } else {
        None
    }
}

fn gen_vec<T>(
    rng: &mut noprop::Rng,
    max_len: usize,
    mut f: impl FnMut(&mut noprop::Rng) -> T,
) -> Vec<T> {
    let n = gen_len(rng, max_len);
    (0..n).map(|_| f(rng)).collect()
}

fn gen_string_arbitrary(rng: &mut noprop::Rng, max_len: usize) -> String {
    let n = gen_len(rng, max_len);
    (0..n).map(|_| noprop::gen_char(rng)).collect()
}

/// ASCII printable, excluding `"` and `\` — mirrors the ASCII character
/// set used by the original proptest `plain_ascii_string` helper.
fn gen_string_ascii_plain(rng: &mut noprop::Rng, min: usize, max: usize) -> String {
    let n = min + (noprop::gen_usize(rng) % (max - min + 1));
    (0..n)
        .map(|_| {
            loop {
                let c = noprop::gen_ascii_printable_char(rng);
                if c != '"' && c != '\\' {
                    return c;
                }
            }
        })
        .collect()
}

/// Analogue of proptest's `mixed_unicode_ascii_string`: any prefix, a
/// guaranteed non-ASCII char, an ASCII run, then any suffix.
fn gen_string_mixed(rng: &mut noprop::Rng) -> String {
    let mut s = gen_string_arbitrary(rng, MAX_LEN);
    let non_ascii = loop {
        let c = noprop::gen_char(rng);
        if !c.is_ascii() {
            break c;
        }
    };
    s.push(non_ascii);
    s.push_str(&gen_string_ascii_plain(rng, 1, MAX_LEN));
    s.push_str(&gen_string_arbitrary(rng, MAX_LEN));
    s
}

fn gen_finite_f32(rng: &mut noprop::Rng) -> f32 {
    loop {
        let v = f32::from_bits(noprop::gen_u32(rng));
        if v.is_finite() {
            return v;
        }
    }
}

fn gen_finite_f64(rng: &mut noprop::Rng) -> f64 {
    loop {
        let v = f64::from_bits(noprop::gen_u64(rng));
        if v.is_finite() {
            return v;
        }
    }
}

// --- Roundtrip tests -------------------------------------------------

#[test]
fn roundtrip_bool() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_bool(rng);
        let parsed: Json<bool> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i8() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_i8(rng);
        let parsed: Json<i8> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i16() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_i16(rng);
        let parsed: Json<i16> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_i32(rng);
        let parsed: Json<i32> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i64() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_i64(rng);
        let parsed: Json<i64> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_i128() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_i128(rng);
        let parsed: Json<i128> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u8() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_u8(rng);
        let parsed: Json<u8> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u16() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_u16(rng);
        let parsed: Json<u16> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u32() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_u32(rng);
        let parsed: Json<u32> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u64() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_u64(rng);
        let parsed: Json<u64> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_u128() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_u128(rng);
        let parsed: Json<u128> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_isize() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_isize(rng);
        let parsed: Json<isize> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_usize() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = noprop::gen_usize(rng);
        let parsed: Json<usize> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_f32_finite() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = gen_finite_f32(rng);
        let parsed: Json<f32> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_f64_finite() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = gen_finite_f64(rng);
        let parsed: Json<f64> = Json(v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_string() -> noprop::Result<()> {
    runner().run(|rng| {
        let s = gen_string_arbitrary(rng, MAX_LEN);
        let parsed: Json<String> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

#[test]
fn roundtrip_string_with_non_ascii_followed_by_ascii() -> noprop::Result<()> {
    runner().run(|rng| {
        let s = gen_string_mixed(rng);
        let parsed: Json<String> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

#[test]
fn roundtrip_char() -> noprop::Result<()> {
    runner().run(|rng| {
        let c = noprop::gen_char(rng);
        let parsed: Json<char> = Json(c).to_string().parse()?;
        assert_eq!(parsed.0, c);
        Ok(())
    })
}

#[test]
fn roundtrip_option_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let opt = gen_option(rng, noprop::gen_i32);
        let parsed: Json<Option<i32>> = Json(opt).to_string().parse()?;
        assert_eq!(parsed.0, opt);
        Ok(())
    })
}

#[test]
fn roundtrip_option_string() -> noprop::Result<()> {
    runner().run(|rng| {
        let opt = gen_option(rng, |rng| gen_string_arbitrary(rng, MAX_LEN));
        let parsed: Json<Option<String>> = Json(opt.as_ref()).to_string().parse()?;
        assert_eq!(parsed.0, opt);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = gen_vec(rng, MAX_LEN, noprop::gen_i32);
        let parsed: Json<Vec<i32>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_string() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = gen_vec(rng, MAX_LEN, |rng| gen_string_arbitrary(rng, MAX_LEN));
        let parsed: Json<Vec<String>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_vec_option_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = gen_vec(rng, MAX_LEN, |rng| gen_option(rng, noprop::gen_i32));
        let parsed: Json<Vec<Option<i32>>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_nested_vec() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = gen_vec(rng, MAX_LEN, |rng| gen_vec(rng, MAX_LEN, noprop::gen_i32));
        let parsed: Json<Vec<Vec<i32>>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_btreemap_string_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let n = gen_len(rng, MAX_LEN);
        let mut m = BTreeMap::new();
        for _ in 0..n {
            m.insert(gen_string_arbitrary(rng, MAX_LEN), noprop::gen_i32(rng));
        }
        let parsed: Json<BTreeMap<String, i32>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}

#[test]
fn roundtrip_btreemap_string_option_string() -> noprop::Result<()> {
    runner().run(|rng| {
        let n = gen_len(rng, MAX_LEN);
        let mut m = BTreeMap::new();
        for _ in 0..n {
            let k = gen_string_arbitrary(rng, MAX_LEN);
            let v = gen_option(rng, |rng| gen_string_arbitrary(rng, MAX_LEN));
            m.insert(k, v);
        }
        let parsed: Json<BTreeMap<String, Option<String>>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}

#[test]
fn roundtrip_array_fixed() -> noprop::Result<()> {
    runner().run(|rng| {
        let arr: [i32; 5] = [
            noprop::gen_i32(rng),
            noprop::gen_i32(rng),
            noprop::gen_i32(rng),
            noprop::gen_i32(rng),
            noprop::gen_i32(rng),
        ];
        let parsed: Json<[i32; 5]> = Json(arr).to_string().parse()?;
        assert_eq!(parsed.0, arr);
        Ok(())
    })
}

// --- NonZero types ---------------------------------------------------

#[test]
fn roundtrip_nonzero_i8() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroI8 = noprop::gen_non_zero_i8(rng);
        let parsed: Json<NonZeroI8> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u8() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroU8 = noprop::gen_non_zero_u8(rng);
        let parsed: Json<NonZeroU8> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i16() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroI16 = noprop::gen_non_zero_i16(rng);
        let parsed: Json<NonZeroI16> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u16() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroU16 = noprop::gen_non_zero_u16(rng);
        let parsed: Json<NonZeroU16> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroI32 = noprop::gen_non_zero_i32(rng);
        let parsed: Json<NonZeroI32> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u32() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroU32 = noprop::gen_non_zero_u32(rng);
        let parsed: Json<NonZeroU32> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i64() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroI64 = noprop::gen_non_zero_i64(rng);
        let parsed: Json<NonZeroI64> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u64() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroU64 = noprop::gen_non_zero_u64(rng);
        let parsed: Json<NonZeroU64> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_i128() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroI128 = noprop::gen_non_zero_i128(rng);
        let parsed: Json<NonZeroI128> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_u128() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroU128 = noprop::gen_non_zero_u128(rng);
        let parsed: Json<NonZeroU128> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_isize() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroIsize = noprop::gen_non_zero_isize(rng);
        let parsed: Json<NonZeroIsize> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

#[test]
fn roundtrip_nonzero_usize() -> noprop::Result<()> {
    runner().run(|rng| {
        let nz: NonZeroUsize = noprop::gen_non_zero_usize(rng);
        let parsed: Json<NonZeroUsize> = Json(nz).to_string().parse()?;
        assert_eq!(parsed.0, nz);
        Ok(())
    })
}

// --- Smart pointers --------------------------------------------------

#[test]
fn roundtrip_box_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let n = noprop::gen_i32(rng);
        let b = Box::new(n);
        let parsed: Json<i32> = Json(&b).to_string().parse()?;
        assert_eq!(parsed.0, n);
        Ok(())
    })
}

#[test]
fn roundtrip_rc_string() -> noprop::Result<()> {
    runner().run(|rng| {
        let s = gen_string_arbitrary(rng, MAX_LEN);
        let r = Rc::new(s.clone());
        let parsed: Json<Rc<String>> = Json(&r).to_string().parse()?;
        assert_eq!(parsed.0.as_ref(), &s);
        Ok(())
    })
}

#[test]
fn roundtrip_arc_string() -> noprop::Result<()> {
    runner().run(|rng| {
        let s = gen_string_arbitrary(rng, MAX_LEN);
        let a = Arc::new(s.clone());
        let parsed: Json<Arc<String>> = Json(&a).to_string().parse()?;
        assert_eq!(parsed.0.as_ref(), &s);
        Ok(())
    })
}

// --- Additional collections ------------------------------------------

#[test]
fn roundtrip_hashmap_string_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let n = gen_len(rng, MAX_LEN);
        let mut m = HashMap::new();
        for _ in 0..n {
            m.insert(gen_string_arbitrary(rng, MAX_LEN), noprop::gen_i32(rng));
        }
        let parsed: Json<HashMap<String, i32>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}

#[test]
fn roundtrip_vecdeque_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let v: VecDeque<i32> = gen_vec(rng, MAX_LEN, noprop::gen_i32).into();
        let parsed: Json<VecDeque<i32>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_btreeset_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let n = gen_len(rng, MAX_LEN);
        let mut s = BTreeSet::new();
        for _ in 0..n {
            s.insert(noprop::gen_i32(rng));
        }
        let parsed: Json<BTreeSet<i32>> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

#[test]
fn roundtrip_hashset_i32() -> noprop::Result<()> {
    runner().run(|rng| {
        let n = gen_len(rng, MAX_LEN);
        let mut s = HashSet::new();
        for _ in 0..n {
            s.insert(noprop::gen_i32(rng));
        }
        let parsed: Json<HashSet<i32>> = Json(&s).to_string().parse()?;
        assert_eq!(parsed.0, s);
        Ok(())
    })
}

// --- PathBuf ---------------------------------------------------------

#[test]
fn roundtrip_pathbuf() -> noprop::Result<()> {
    runner().run(|rng| {
        let s = gen_string_arbitrary(rng, MAX_LEN);
        let p = PathBuf::from(&s);
        let parsed: Json<PathBuf> = Json(&p).to_string().parse()?;
        assert_eq!(parsed.0, p);
        Ok(())
    })
}

// --- Network types ---------------------------------------------------

#[test]
fn roundtrip_ipv4addr() -> noprop::Result<()> {
    runner().run(|rng| {
        let ip = Ipv4Addr::new(
            noprop::gen_u8(rng),
            noprop::gen_u8(rng),
            noprop::gen_u8(rng),
            noprop::gen_u8(rng),
        );
        let parsed: Json<Ipv4Addr> = Json(ip).to_string().parse()?;
        assert_eq!(parsed.0, ip);
        Ok(())
    })
}

#[test]
fn roundtrip_ipv6addr() -> noprop::Result<()> {
    runner().run(|rng| {
        let ip = Ipv6Addr::new(
            noprop::gen_u16(rng),
            noprop::gen_u16(rng),
            noprop::gen_u16(rng),
            noprop::gen_u16(rng),
            noprop::gen_u16(rng),
            noprop::gen_u16(rng),
            noprop::gen_u16(rng),
            noprop::gen_u16(rng),
        );
        let parsed: Json<Ipv6Addr> = Json(ip).to_string().parse()?;
        assert_eq!(parsed.0, ip);
        Ok(())
    })
}

#[test]
fn roundtrip_ipaddr_v4() -> noprop::Result<()> {
    runner().run(|rng| {
        let ip = IpAddr::V4(Ipv4Addr::new(
            noprop::gen_u8(rng),
            noprop::gen_u8(rng),
            noprop::gen_u8(rng),
            noprop::gen_u8(rng),
        ));
        let parsed: Json<IpAddr> = Json(ip).to_string().parse()?;
        assert_eq!(parsed.0, ip);
        Ok(())
    })
}

#[test]
fn roundtrip_socketaddr_v4() -> noprop::Result<()> {
    runner().run(|rng| {
        let addr = SocketAddrV4::new(
            Ipv4Addr::new(
                noprop::gen_u8(rng),
                noprop::gen_u8(rng),
                noprop::gen_u8(rng),
                noprop::gen_u8(rng),
            ),
            noprop::gen_u16(rng),
        );
        let parsed: Json<SocketAddrV4> = Json(addr).to_string().parse()?;
        assert_eq!(parsed.0, addr);
        Ok(())
    })
}

#[test]
fn roundtrip_socketaddr_v6() -> noprop::Result<()> {
    runner().run(|rng| {
        let addr = SocketAddrV6::new(
            Ipv6Addr::new(
                noprop::gen_u16(rng),
                noprop::gen_u16(rng),
                noprop::gen_u16(rng),
                noprop::gen_u16(rng),
                noprop::gen_u16(rng),
                noprop::gen_u16(rng),
                noprop::gen_u16(rng),
                noprop::gen_u16(rng),
            ),
            noprop::gen_u16(rng),
            0,
            0,
        );
        let parsed: Json<SocketAddrV6> = Json(addr).to_string().parse()?;
        assert_eq!(parsed.0, addr);
        Ok(())
    })
}

#[test]
fn roundtrip_socketaddr() -> noprop::Result<()> {
    runner().run(|rng| {
        let addr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(
                noprop::gen_u8(rng),
                noprop::gen_u8(rng),
                noprop::gen_u8(rng),
                noprop::gen_u8(rng),
            ),
            noprop::gen_u16(rng),
        ));
        let parsed: Json<SocketAddr> = Json(addr).to_string().parse()?;
        assert_eq!(parsed.0, addr);
        Ok(())
    })
}

// --- Unit ------------------------------------------------------------

#[test]
fn roundtrip_unit() -> noprop::Result<()> {
    runner().run(|_rng| {
        let parsed: Json<()> = Json(()).to_string().parse()?;
        assert_eq!(parsed.0, ());
        Ok(())
    })
}

// --- Deeply nested ---------------------------------------------------

#[test]
fn roundtrip_vec_btreemap() -> noprop::Result<()> {
    runner().run(|rng| {
        let v = gen_vec(rng, MAX_LEN, |rng| {
            let n = gen_len(rng, MAX_LEN);
            let mut m = BTreeMap::new();
            for _ in 0..n {
                m.insert(gen_string_arbitrary(rng, MAX_LEN), noprop::gen_i32(rng));
            }
            m
        });
        let parsed: Json<Vec<BTreeMap<String, i32>>> = Json(&v).to_string().parse()?;
        assert_eq!(parsed.0, v);
        Ok(())
    })
}

#[test]
fn roundtrip_btreemap_vec() -> noprop::Result<()> {
    runner().run(|rng| {
        let n = gen_len(rng, MAX_LEN);
        let mut m = BTreeMap::new();
        for _ in 0..n {
            let k = gen_string_arbitrary(rng, MAX_LEN);
            let v = gen_vec(rng, MAX_LEN, noprop::gen_i32);
            m.insert(k, v);
        }
        let parsed: Json<BTreeMap<String, Vec<i32>>> = Json(&m).to_string().parse()?;
        assert_eq!(parsed.0, m);
        Ok(())
    })
}
