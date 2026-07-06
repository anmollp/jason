use criterion::{Criterion, criterion_group, criterion_main};
use jason::{parse_from_str, to_json_string, to_pretty_string};
use std::hint::black_box;
mod common;
use common::large_json;

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
