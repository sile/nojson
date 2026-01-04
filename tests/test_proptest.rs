use std::collections::BTreeMap;

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
}
