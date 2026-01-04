use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use nojson::Json;
use proptest::prelude::*;

proptest! {
    #[test]
    fn roundtrip_bool(b: bool) {
        let json_str = Json(b).to_string();
        let parsed: Json<bool> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, b);
    }

    #[test]
    fn roundtrip_i8(n: i8) {
        let json_str = Json(n).to_string();
        let parsed: Json<i8> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_i16(n: i16) {
        let json_str = Json(n).to_string();
        let parsed: Json<i16> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_i32(n: i32) {
        let json_str = Json(n).to_string();
        let parsed: Json<i32> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_i64(n: i64) {
        let json_str = Json(n).to_string();
        let parsed: Json<i64> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_i128(n: i128) {
        let json_str = Json(n).to_string();
        let parsed: Json<i128> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_u8(n: u8) {
        let json_str = Json(n).to_string();
        let parsed: Json<u8> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_u16(n: u16) {
        let json_str = Json(n).to_string();
        let parsed: Json<u16> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_u32(n: u32) {
        let json_str = Json(n).to_string();
        let parsed: Json<u32> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_u64(n: u64) {
        let json_str = Json(n).to_string();
        let parsed: Json<u64> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_u128(n: u128) {
        let json_str = Json(n).to_string();
        let parsed: Json<u128> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_f32_finite(n in prop::num::f32::NORMAL | prop::num::f32::SUBNORMAL | prop::num::f32::ZERO) {
        let json_str = Json(n).to_string();
        let parsed: Json<f32> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_f64_finite(n in prop::num::f64::NORMAL | prop::num::f64::SUBNORMAL | prop::num::f64::ZERO) {
        let json_str = Json(n).to_string();
        let parsed: Json<f64> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_string(s: String) {
        let json_str = Json(&s).to_string();
        let parsed: Json<String> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, s);
    }

    #[test]
    fn roundtrip_char(c: char) {
        let json_str = Json(c).to_string();
        let parsed: Json<char> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, c);
    }

    #[test]
    fn roundtrip_option_i32(opt: Option<i32>) {
        let json_str = Json(opt).to_string();
        let parsed: Json<Option<i32>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, opt);
    }

    #[test]
    fn roundtrip_option_string(opt: Option<String>) {
        let json_str = Json(opt.as_ref()).to_string();
        let parsed: Json<Option<String>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, opt);
    }

    #[test]
    fn roundtrip_vec_i32(v: Vec<i32>) {
        let json_str = Json(&v).to_string();
        let parsed: Json<Vec<i32>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, v);
    }

    #[test]
    fn roundtrip_vec_string(v: Vec<String>) {
        let json_str = Json(&v).to_string();
        let parsed: Json<Vec<String>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, v);
    }

    #[test]
    fn roundtrip_vec_option_i32(v: Vec<Option<i32>>) {
        let json_str = Json(&v).to_string();
        let parsed: Json<Vec<Option<i32>>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, v);
    }

    #[test]
    fn roundtrip_nested_vec(v: Vec<Vec<i32>>) {
        let json_str = Json(&v).to_string();
        let parsed: Json<Vec<Vec<i32>>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, v);
    }

    #[test]
    fn roundtrip_btreemap_string_i32(m: BTreeMap<String, i32>) {
        let json_str = Json(&m).to_string();
        let parsed: Json<BTreeMap<String, i32>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, m);
    }

    #[test]
    fn roundtrip_btreemap_string_option_string(m: BTreeMap<String, Option<String>>) {
        let json_str = Json(&m).to_string();
        let parsed: Json<BTreeMap<String, Option<String>>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, m);
    }

    #[test]
    fn roundtrip_array_fixed(arr: [i32; 5]) {
        let json_str = Json(arr).to_string();
        let parsed: Json<[i32; 5]> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, arr);
    }

    // isize/usize
    #[test]
    fn roundtrip_isize(n: isize) {
        let json_str = Json(n).to_string();
        let parsed: Json<isize> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_usize(n: usize) {
        let json_str = Json(n).to_string();
        let parsed: Json<usize> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    // NonZero types
    #[test]
    fn roundtrip_nonzero_i8(n in any::<i8>().prop_filter("non-zero", |&x| x != 0)) {
        let nz = NonZeroI8::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroI8> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_u8(n in 1u8..=u8::MAX) {
        let nz = NonZeroU8::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroU8> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_i16(n in any::<i16>().prop_filter("non-zero", |&x| x != 0)) {
        let nz = NonZeroI16::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroI16> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_u16(n in 1u16..=u16::MAX) {
        let nz = NonZeroU16::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroU16> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_i32(n in any::<i32>().prop_filter("non-zero", |&x| x != 0)) {
        let nz = NonZeroI32::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroI32> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_u32(n in 1u32..=u32::MAX) {
        let nz = NonZeroU32::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroU32> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_i64(n in any::<i64>().prop_filter("non-zero", |&x| x != 0)) {
        let nz = NonZeroI64::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroI64> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_u64(n in 1u64..=u64::MAX) {
        let nz = NonZeroU64::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroU64> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_i128(n in any::<i128>().prop_filter("non-zero", |&x| x != 0)) {
        let nz = NonZeroI128::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroI128> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_u128(n in 1u128..=u128::MAX) {
        let nz = NonZeroU128::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroU128> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_isize(n in any::<isize>().prop_filter("non-zero", |&x| x != 0)) {
        let nz = NonZeroIsize::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroIsize> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    #[test]
    fn roundtrip_nonzero_usize(n in 1usize..=usize::MAX) {
        let nz = NonZeroUsize::new(n).unwrap();
        let json_str = Json(nz).to_string();
        let parsed: Json<NonZeroUsize> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, nz);
    }

    // Smart pointers
    #[test]
    fn roundtrip_box_i32(n: i32) {
        let b = Box::new(n);
        let json_str = Json(&b).to_string();
        let parsed: Json<i32> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, n);
    }

    #[test]
    fn roundtrip_rc_string(s: String) {
        let r = Rc::new(s.clone());
        let json_str = Json(&r).to_string();
        let parsed: Json<Rc<String>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0.as_ref(), &s);
    }

    #[test]
    fn roundtrip_arc_string(s: String) {
        let a = Arc::new(s.clone());
        let json_str = Json(&a).to_string();
        let parsed: Json<Arc<String>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0.as_ref(), &s);
    }

    // Collections
    #[test]
    fn roundtrip_hashmap_string_i32(m: HashMap<String, i32>) {
        let json_str = Json(&m).to_string();
        let parsed: Json<HashMap<String, i32>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, m);
    }

    #[test]
    fn roundtrip_vecdeque_i32(v: VecDeque<i32>) {
        let json_str = Json(&v).to_string();
        let parsed: Json<VecDeque<i32>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, v);
    }

    #[test]
    fn roundtrip_btreeset_i32(s: BTreeSet<i32>) {
        let json_str = Json(&s).to_string();
        let parsed: Json<BTreeSet<i32>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, s);
    }

    #[test]
    fn roundtrip_hashset_i32(s: HashSet<i32>) {
        let json_str = Json(&s).to_string();
        let parsed: Json<HashSet<i32>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, s);
    }

    // PathBuf
    #[test]
    fn roundtrip_pathbuf(s: String) {
        let p = PathBuf::from(&s);
        let json_str = Json(&p).to_string();
        let parsed: Json<PathBuf> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, p);
    }

    // Network types
    #[test]
    fn roundtrip_ipv4addr(a: u8, b: u8, c: u8, d: u8) {
        let ip = Ipv4Addr::new(a, b, c, d);
        let json_str = Json(ip).to_string();
        let parsed: Json<Ipv4Addr> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, ip);
    }

    #[test]
    fn roundtrip_ipv6addr(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) {
        let ip = Ipv6Addr::new(a, b, c, d, e, f, g, h);
        let json_str = Json(ip).to_string();
        let parsed: Json<Ipv6Addr> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, ip);
    }

    #[test]
    fn roundtrip_ipaddr_v4(a: u8, b: u8, c: u8, d: u8) {
        let ip = IpAddr::V4(Ipv4Addr::new(a, b, c, d));
        let json_str = Json(ip).to_string();
        let parsed: Json<IpAddr> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, ip);
    }

    #[test]
    fn roundtrip_socketaddr_v4(a: u8, b: u8, c: u8, d: u8, port: u16) {
        let addr = SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port);
        let json_str = Json(addr).to_string();
        let parsed: Json<SocketAddrV4> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, addr);
    }

    #[test]
    fn roundtrip_socketaddr_v6(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16, port: u16) {
        let addr = SocketAddrV6::new(Ipv6Addr::new(a, b, c, d, e, f, g, h), port, 0, 0);
        let json_str = Json(addr).to_string();
        let parsed: Json<SocketAddrV6> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, addr);
    }

    #[test]
    fn roundtrip_socketaddr(a: u8, b: u8, c: u8, d: u8, port: u16) {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port));
        let json_str = Json(addr).to_string();
        let parsed: Json<SocketAddr> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, addr);
    }

    // Unit type
    #[test]
    fn roundtrip_unit(_x in Just(())) {
        let json_str = Json(()).to_string();
        let parsed: Json<()> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, ());
    }

    // Deeply nested structures
    #[test]
    fn roundtrip_vec_btreemap(v: Vec<BTreeMap<String, i32>>) {
        let json_str = Json(&v).to_string();
        let parsed: Json<Vec<BTreeMap<String, i32>>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, v);
    }

    #[test]
    fn roundtrip_btreemap_vec(m: BTreeMap<String, Vec<i32>>) {
        let json_str = Json(&m).to_string();
        let parsed: Json<BTreeMap<String, Vec<i32>>> = json_str.parse().unwrap();
        prop_assert_eq!(parsed.0, m);
    }
}
