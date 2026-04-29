//! Parse and format benchmarks for nojson.
//!
//! Run with: `cargo run --release --example benchmark`
//!
//! Optional env var `BENCH_REPEATS` (default 30) controls the number of
//! best-of-N samples per case. Each sample auto-scales its inner loop count
//! to land near 100 ms of wall time, so reported numbers stay stable across
//! the size range covered here (a few hundred bytes through a few KB).
//!
//! Uses `std::time::Instant` rather than `criterion` so the crate keeps a
//! lean dev-dep tree.

use std::time::{Duration, Instant};

const TARGET_DURATION: Duration = Duration::from_millis(100);

struct Row {
    name: String,
    bytes: usize,
    ns_per_op: f64,
}

fn measure_one<F: FnMut()>(mut op: F) -> f64 {
    for _ in 0..3 {
        op();
    }
    let mut iters: u64 = 1;
    loop {
        let start = Instant::now();
        for _ in 0..iters {
            op();
        }
        let elapsed = start.elapsed();
        if elapsed >= TARGET_DURATION {
            return elapsed.as_nanos() as f64 / iters as f64;
        }
        let factor =
            (TARGET_DURATION.as_nanos() as f64 / elapsed.as_nanos().max(1) as f64).max(2.0);
        iters = (iters as f64 * factor).ceil() as u64;
    }
}

fn best_of<F: FnMut() -> f64>(repeats: usize, mut measure: F) -> f64 {
    (0..repeats)
        .map(|_| measure())
        .fold(f64::INFINITY, f64::min)
}

fn print_section(title: &str, rows: &[Row]) {
    println!("=== {title} ===");
    println!(
        "  {:<32} {:>10} {:>14} {:>12}",
        "case", "size", "ns/op", "MB/s"
    );
    for r in rows {
        let mb_s = if r.ns_per_op > 0.0 {
            r.bytes as f64 / r.ns_per_op * 1_000.0
        } else {
            f64::INFINITY
        };
        let size = format!("{} B", r.bytes);
        println!(
            "  {:<32} {:>10} {:>14.1} {:>12.1}",
            r.name, size, r.ns_per_op, mb_s
        );
    }
    println!();
}

// ----- Parse input generators -----

fn gen_long_ascii(len: usize) -> String {
    let base = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ,.:;!?-_";
    let body: String = base.chars().cycle().take(len).collect();
    format!(r#""{body}""#)
}

fn gen_ascii_with_escapes(len: usize, interval: usize) -> String {
    let mut body = String::with_capacity(len + len / interval * 2);
    let base = "abcdefghijklmnopqrstuvwxyz";
    let mut chars = base.chars().cycle();
    for i in 0..len {
        if i > 0 && i % interval == 0 {
            body.push_str(r#"\""#);
        } else {
            body.push(chars.next().unwrap());
        }
    }
    format!(r#""{body}""#)
}

fn gen_many_short_keys(count: usize) -> String {
    let mut s = String::from("{");
    for i in 0..count {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&format!(r#""key{i}":"val{i}""#));
    }
    s.push('}');
    s
}

fn gen_unicode_heavy_quoted(len: usize) -> String {
    let chars = "あいうえおかきくけこ日本語テスト🎉🚀✨";
    let body: String = chars.chars().cycle().take(len).collect();
    format!(r#""{body}""#)
}

fn gen_unicode_escapes(count: usize) -> String {
    let mut s = String::with_capacity(2 + count * 6);
    s.push('"');
    for i in 0..count {
        let code = match i % 4 {
            0 => "3042", // あ
            1 => "65e5", // 日
            2 => "672c", // 本
            _ => "8a9e", // 語
        };
        s.push_str(r#"\u"#);
        s.push_str(code);
    }
    s.push('"');
    s
}

fn gen_int_array(count: usize) -> String {
    let mut s = String::from("[");
    for i in 0..count {
        if i > 0 {
            s.push(',');
        }
        // Mix small and larger integers, plus a few negatives.
        let n: i64 = match i % 5 {
            0 => i as i64,
            1 => -(i as i64),
            2 => (i as i64) * 1_000,
            3 => (i as i64) * 1_000_000,
            _ => (i as i64) - 50,
        };
        s.push_str(&n.to_string());
    }
    s.push(']');
    s
}

fn gen_float_array(count: usize) -> String {
    let mut s = String::from("[");
    for i in 0..count {
        if i > 0 {
            s.push(',');
        }
        // Mix plain decimals, exponents, negatives, and small magnitudes.
        match i % 5 {
            0 => s.push_str(&format!("{:.4}", i as f64 * 1.5)),
            1 => s.push_str(&format!("{:e}", i as f64 * 2.7e-3 + 1.0)),
            2 => s.push_str(&format!("-{:.6}", i as f64 * 0.001)),
            3 => s.push_str(&format!("{:.2}", i as f64 + 0.25)),
            _ => s.push_str(&format!("{:.3e}", i as f64 * 1.234e6 + 1.0)),
        }
    }
    s.push(']');
    s
}

fn gen_full_json_document() -> String {
    let mut s = String::from(r#"{"users":["#);
    for i in 0..50 {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&format!(
            r#"{{"id":{i},"name":"User {i} with a reasonably long name for testing","email":"user{i}@example.com","bio":"This is a biography text that contains several sentences. It is meant to test the performance of string parsing with typical content. Nothing special here, just plain ASCII text that goes on for a while to provide a realistic benchmark scenario.","active":true,"score":98.6}}"#
        ));
    }
    s.push_str("]}");
    s
}

fn gen_jsonc_document() -> String {
    // Realistic JSONC: line + block comments, trailing commas, light indent.
    let mut s = String::from("// User export, generated for benchmarks.\n{\n  \"users\": [\n");
    for i in 0..50 {
        if i > 0 && i % 10 == 0 {
            s.push_str(&format!("    /* batch boundary at {i} */\n"));
        }
        s.push_str(&format!(
            "    // user #{i}\n    {{\"id\":{i},\"name\":\"User {i} with a reasonably long name for testing\",\"email\":\"user{i}@example.com\",\"bio\":\"This is a biography text that contains several sentences. It is meant to test the performance of string parsing with typical content. Nothing special here, just plain ASCII text that goes on for a while to provide a realistic benchmark scenario.\",\"active\":true,\"score\":98.6}},\n"
        ));
    }
    s.push_str("  ],\n}\n");
    s
}

// ----- Format input generators -----

fn gen_plain_ascii(len: usize) -> String {
    let base = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ,.:;!?-_";
    base.chars().cycle().take(len).collect()
}

fn gen_mixed_escapes(len: usize) -> String {
    let base = "hello world\nthis has \"quotes\" and \\backslashes\r\nand\ttabs";
    base.chars().cycle().take(len).collect()
}

fn gen_unicode_heavy_raw(len: usize) -> String {
    let chars = "あいうえおかきくけこ日本語テスト🎉🚀✨";
    chars.chars().cycle().take(len).collect()
}

fn gen_int_vec(count: usize) -> Vec<i64> {
    (0..count)
        .map(|i| match i % 5 {
            0 => i as i64,
            1 => -(i as i64),
            2 => (i as i64) * 1_000,
            3 => (i as i64) * 1_000_000,
            _ => (i as i64) - 50,
        })
        .collect()
}

fn gen_float_vec(count: usize) -> Vec<f64> {
    (0..count)
        .map(|i| match i % 5 {
            0 => i as f64 * 1.5,
            1 => i as f64 * 2.7e-3 + 1.0,
            2 => -(i as f64) * 0.001,
            3 => i as f64 + 0.25,
            _ => i as f64 * 1.234e6 + 1.0,
        })
        .collect()
}

struct UserRecord {
    id: i32,
    name: String,
    email: String,
    bio: String,
    active: bool,
    score: f64,
    tags: Vec<&'static str>,
}

fn gen_user_records(count: usize) -> Vec<UserRecord> {
    (0..count)
        .map(|i| UserRecord {
            id: i as i32,
            name: format!("User {i} with a reasonably long name for testing"),
            email: format!("user{i}@example.com"),
            bio: "This is a biography text that contains several sentences. It is meant to test the performance of string parsing with typical content. Nothing special here, just plain ASCII text that goes on for a while to provide a realistic benchmark scenario.".to_string(),
            active: i % 3 != 0,
            score: 98.6 + i as f64 * 0.01,
            tags: match i % 3 {
                0 => vec!["alpha", "beta"],
                1 => vec!["gamma"],
                _ => vec!["alpha", "delta", "epsilon"],
            },
        })
        .collect()
}

// ----- Sections -----

fn bench_parse(repeats: usize) -> Vec<Row> {
    let cases: Vec<(String, String)> = {
        let mut v: Vec<(String, String)> = Vec::new();
        for &len in &[64usize, 256, 1024] {
            v.push((format!("long_ascii_no_escape_{len}B"), gen_long_ascii(len)));
        }
        v.push((
            "ascii_with_escapes_256B/32".into(),
            gen_ascii_with_escapes(256, 32),
        ));
        v.push(("many_short_keys_100".into(), gen_many_short_keys(100)));
        v.push(("unicode_heavy_200ch".into(), gen_unicode_heavy_quoted(200)));
        v.push(("unicode_escapes_128".into(), gen_unicode_escapes(128)));
        v.push(("int_array_1000".into(), gen_int_array(1000)));
        v.push(("float_array_1000".into(), gen_float_array(1000)));
        v.push((
            "full_json_document_50users".into(),
            gen_full_json_document(),
        ));
        v
    };

    let mut rows: Vec<Row> = cases
        .iter()
        .map(|(name, input)| {
            let bytes = input.len();
            let ns = best_of(repeats, || {
                measure_one(|| {
                    let _ = nojson::RawJson::parse(input).unwrap();
                })
            });
            Row {
                name: name.clone(),
                bytes,
                ns_per_op: ns,
            }
        })
        .collect();

    // JSONC parse via the dedicated entry point.
    let jsonc = gen_jsonc_document();
    let bytes = jsonc.len();
    let ns = best_of(repeats, || {
        measure_one(|| {
            let _ = nojson::RawJson::parse_jsonc(&jsonc).unwrap();
        })
    });
    rows.push(Row {
        name: "jsonc_document_50users".into(),
        bytes,
        ns_per_op: ns,
    });

    rows
}

fn bench_format(repeats: usize) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();

    for &len in &[64usize, 256, 1024] {
        let input = gen_plain_ascii(len);
        let out_len = nojson::json(|f| f.value(input.as_str())).to_string().len();
        let ns = best_of(repeats, || {
            measure_one(|| {
                let _ = nojson::json(|f| f.value(input.as_str())).to_string();
            })
        });
        rows.push(Row {
            name: format!("plain_ascii_{len}B"),
            bytes: out_len,
            ns_per_op: ns,
        });
    }

    let input = gen_mixed_escapes(256);
    let out_len = nojson::json(|f| f.value(input.as_str())).to_string().len();
    let ns = best_of(repeats, || {
        measure_one(|| {
            let _ = nojson::json(|f| f.value(input.as_str())).to_string();
        })
    });
    rows.push(Row {
        name: "mixed_escapes_256B".into(),
        bytes: out_len,
        ns_per_op: ns,
    });

    let input = gen_unicode_heavy_raw(200);
    let out_len = nojson::json(|f| f.value(input.as_str())).to_string().len();
    let ns = best_of(repeats, || {
        measure_one(|| {
            let _ = nojson::json(|f| f.value(input.as_str())).to_string();
        })
    });
    rows.push(Row {
        name: "unicode_heavy_200ch".into(),
        bytes: out_len,
        ns_per_op: ns,
    });

    // Object with many string key-value pairs.
    let keys: Vec<String> = (0..50).map(|i| format!("key_{i}")).collect();
    let values: Vec<String> = (0..50)
        .map(|i| {
            format!("This is value number {i} with a reasonably long text content for benchmarking")
        })
        .collect();
    let pairs: Vec<(&str, &str)> = keys
        .iter()
        .zip(values.iter())
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let out_len = nojson::json(|f| {
        f.object(|f| {
            for &(k, v) in pairs.iter() {
                f.member(k, v)?;
            }
            Ok(())
        })
    })
    .to_string()
    .len();
    let ns = best_of(repeats, || {
        measure_one(|| {
            let _ = nojson::json(|f| {
                f.object(|f| {
                    for &(k, v) in pairs.iter() {
                        f.member(k, v)?;
                    }
                    Ok(())
                })
            })
            .to_string();
        })
    });
    rows.push(Row {
        name: "object_formatting_50pairs".into(),
        bytes: out_len,
        ns_per_op: ns,
    });

    // Numeric arrays.
    let ints = gen_int_vec(1000);
    let out_len = nojson::json(|f| f.value(&ints)).to_string().len();
    let ns = best_of(repeats, || {
        measure_one(|| {
            let _ = nojson::json(|f| f.value(&ints)).to_string();
        })
    });
    rows.push(Row {
        name: "int_array_1000".into(),
        bytes: out_len,
        ns_per_op: ns,
    });

    let floats = gen_float_vec(1000);
    let out_len = nojson::json(|f| f.value(&floats)).to_string().len();
    let ns = best_of(repeats, || {
        measure_one(|| {
            let _ = nojson::json(|f| f.value(&floats)).to_string();
        })
    });
    rows.push(Row {
        name: "float_array_1000".into(),
        bytes: out_len,
        ns_per_op: ns,
    });

    // Mixed-type document: parallel to the parse-side full_json_document case
    // but exercises the full formatter (numbers, strings, bools, nested arrays).
    let users = gen_user_records(50);
    let format_users = |f: &mut nojson::JsonFormatter<'_, '_>| {
        f.object(|f| {
            f.member(
                "users",
                nojson::json(|f| {
                    f.array(|f| {
                        for u in &users {
                            f.element(nojson::json(|f| {
                                f.object(|f| {
                                    f.member("id", u.id)?;
                                    f.member("name", u.name.as_str())?;
                                    f.member("email", u.email.as_str())?;
                                    f.member("bio", u.bio.as_str())?;
                                    f.member("active", u.active)?;
                                    f.member("score", u.score)?;
                                    f.member("tags", &u.tags)
                                })
                            }))?;
                        }
                        Ok(())
                    })
                }),
            )
        })
    };
    let out_len = nojson::json(format_users).to_string().len();
    let ns = best_of(repeats, || {
        measure_one(|| {
            let _ = nojson::json(format_users).to_string();
        })
    });
    rows.push(Row {
        name: "mixed_document_50users".into(),
        bytes: out_len,
        ns_per_op: ns,
    });

    rows
}

fn main() {
    let repeats: usize = std::env::var("BENCH_REPEATS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    println!("nojson benchmark");
    println!("repeats per case: {repeats} (best-of-N reported)");
    println!();

    let rows = bench_parse(repeats);
    print_section("parse", &rows);

    let rows = bench_format(repeats);
    print_section("format", &rows);
}
