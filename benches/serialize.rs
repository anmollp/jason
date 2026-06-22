use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use jason::{to_pretty_string, to_json_string, parse_from_str};

fn large_json() -> String {
    let mut s = String::from(r#"{"users":["#);

    for i in 0..10_000 {
        if i > 0 {
            s.push(',');
        }

        s.push_str(&format!(
            r#"{{"id":{},"name":"user{}","active":true}}"#,
            i, i
        ));
    }

    s.push_str("]}");
    s
}

fn bench_serialize(c: &mut Criterion) {
    let json = large_json();
    let value = parse_from_str(&json).unwrap();

    c.bench_function("to_json_string", |b| {
        b.iter(|| {
            black_box(to_json_string(black_box(&value)));
        })
    });

    c.bench_function("to_pretty_string", |b| {
        b.iter(|| {
            black_box(to_pretty_string(black_box(&value)));
        })
    });
}

criterion_group!(benches, bench_serialize);
criterion_main!(benches);