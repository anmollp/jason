use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use jason::parse_from_str;
mod common;
use common::large_json;

fn bench_json_pointer(c: &mut Criterion) {
    let json = large_json();
    let value = parse_from_str(&json).unwrap();

    c.bench_function("pointer_large_depth", |b| {
        b.iter(|| {
            let result = value.pointer("/users/9999/name");
            assert!(result.is_some());
            black_box(result);
        })
    });

    c.bench_function("pointer_no_depth", |b| {
        b.iter(|| {
            let result = value.pointer("/users/0/name");
            assert!(result.is_some());
            black_box(result);
        })
    });

    c.bench_function("pointer_moderate_depth", |b| {
        b.iter(|| {
            let result = value.pointer("/users/100/name");
            assert!(result.is_some());
            black_box(result);
        })
    });
}

criterion_group!(benches, bench_json_pointer);
criterion_main!(benches);