use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn gen_long_ascii(len: usize) -> String {
    let base = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ,.:;!?-_";
    let body: String = base.chars().cycle().take(len).collect();
    format!(r#""{}""#, body)
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
    format!(r#""{}""#, body)
}

fn gen_many_short_keys(count: usize) -> String {
    let mut s = String::from("{");
    for i in 0..count {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&format!(r#""key{}":"val{}""#, i, i));
    }
    s.push('}');
    s
}

fn gen_unicode_heavy(len: usize) -> String {
    let chars = "あいうえおかきくけこ日本語テスト🎉🚀✨";
    let body: String = chars.chars().cycle().take(len).collect();
    format!(r#""{}""#, body)
}

fn gen_full_json_document() -> String {
    let mut s = String::from(r#"{"users":["#);
    for i in 0..50 {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&format!(
            r#"{{"id":{},"name":"User {} with a reasonably long name for testing","email":"user{}@example.com","bio":"This is a biography text that contains several sentences. It is meant to test the performance of string parsing with typical content. Nothing special here, just plain ASCII text that goes on for a while to provide a realistic benchmark scenario.","active":true,"score":98.6}}"#,
            i, i, i
        ));
    }
    s.push_str("]}");
    s
}

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    // Short ASCII string
    let input = r#""hello""#.to_string();
    group.bench_with_input(BenchmarkId::new("short_ascii", "5B"), &input, |b, input| {
        b.iter(|| nojson::RawJson::parse(input).unwrap());
    });

    // Long ASCII, no escapes
    for &len in &[64, 256, 1024] {
        let input = gen_long_ascii(len);
        group.bench_with_input(
            BenchmarkId::new("long_ascii_no_escape", format!("{}B", len)),
            &input,
            |b, input| {
                b.iter(|| nojson::RawJson::parse(input).unwrap());
            },
        );
    }

    // ASCII with periodic escapes
    let input = gen_ascii_with_escapes(256, 32);
    group.bench_with_input(
        BenchmarkId::new("ascii_with_escapes", "256B/32"),
        &input,
        |b, input| {
            b.iter(|| nojson::RawJson::parse(input).unwrap());
        },
    );

    // Many short keys
    let input = gen_many_short_keys(100);
    group.bench_with_input(
        BenchmarkId::new("many_short_keys", "100"),
        &input,
        |b, input| {
            b.iter(|| nojson::RawJson::parse(input).unwrap());
        },
    );

    // Unicode heavy
    let input = gen_unicode_heavy(200);
    group.bench_with_input(
        BenchmarkId::new("unicode_heavy", "200ch"),
        &input,
        |b, input| {
            b.iter(|| nojson::RawJson::parse(input).unwrap());
        },
    );

    // Full JSON document
    let input = gen_full_json_document();
    group.bench_with_input(
        BenchmarkId::new("full_json_document", "50users"),
        &input,
        |b, input| {
            b.iter(|| nojson::RawJson::parse(input).unwrap());
        },
    );

    group.finish();
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
