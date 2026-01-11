#![no_main]

use libfuzzer_sys::fuzz_target;
use nojson::{Json, RawJson};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };

    // Fuzz JSON parsing
    if let Ok(raw) = RawJson::parse(text) {
        let value = raw.value();

        // Try to convert to various types
        let _ = value.as_raw_str();
        let _ = value.kind();
        let _ = value.position();

        // Try type conversions
        let _: Result<bool, _> = value.try_into();
        let _: Result<i64, _> = value.try_into();
        let _: Result<f64, _> = value.try_into();
        let _: Result<String, _> = value.try_into();
        let _: Result<(), _> = value.try_into();
    }

    // Fuzz JSONC parsing
    if let Ok((raw, _comments)) = RawJson::parse_jsonc(text) {
        let _ = raw.value().as_raw_str();
    }

    // Fuzz roundtrip for strings
    let _: Result<Json<String>, _> = text.parse();
    let _: Result<Json<i64>, _> = text.parse();
    let _: Result<Json<f64>, _> = text.parse();
    let _: Result<Json<bool>, _> = text.parse();
    let _: Result<Json<Vec<i32>>, _> = text.parse();
});
