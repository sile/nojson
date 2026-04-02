use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn gen_plain_ascii(len: usize) -> String {
    let base = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ,.:;!?-_";
    base.chars().cycle().take(len).collect()
}

fn gen_mixed_escapes(len: usize) -> String {
    let base = "hello world\nthis has \"quotes\" and \\backslashes\r\nand\ttabs";
    base.chars().cycle().take(len).collect()
}

fn bench_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("format");

    // Plain ASCII string formatting
    for &len in &[64, 256, 1024] {
        let input = gen_plain_ascii(len);
        group.bench_with_input(
            BenchmarkId::new("plain_ascii", format!("{}B", len)),
            &input,
            |b, input| {
                b.iter(|| nojson::json(|f| f.value(input.as_str())).to_string());
            },
        );
    }

    // Mixed escapes
    let input = gen_mixed_escapes(256);
    group.bench_with_input(
        BenchmarkId::new("mixed_escapes", "256B"),
        &input,
        |b, input| {
            b.iter(|| nojson::json(|f| f.value(input.as_str())).to_string());
        },
    );

    // Object with many string key-value pairs
    let keys: Vec<String> = (0..50).map(|i| format!("key_{}", i)).collect();
    let values: Vec<String> = (0..50)
        .map(|i| {
            format!(
                "This is value number {} with a reasonably long text content for benchmarking",
                i
            )
        })
        .collect();
    let pairs: Vec<(&str, &str)> = keys
        .iter()
        .zip(values.iter())
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    group.bench_with_input(
        BenchmarkId::new("object_formatting", "50pairs"),
        &pairs,
        |b, pairs| {
            b.iter(|| {
                nojson::json(|f| {
                    f.object(|f| {
                        for &(k, v) in pairs.iter() {
                            f.member(k, v)?;
                        }
                        Ok(())
                    })
                })
                .to_string()
            });
        },
    );

    group.finish();
}

criterion_group!(benches, bench_format);
criterion_main!(benches);
