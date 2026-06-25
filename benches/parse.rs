use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use jason::parse_from_str;
mod common;
use common::{large_json, medium_json};

fn bench_parse(c: &mut Criterion) {
    let small = r#"
    {
        "name": "Jason",
        "age": 30,
        "skills": ["Rowing", "Cooking", "Pole vaulting"],
        "active": true
    }
    "#;

    let medium = medium_json();
    let large = large_json();

    c.bench_function("parse_small_json", |b| {
        b.iter(|| {
            parse_from_str(black_box(small)).unwrap();
        })
    });

    c.bench_function("parse_medium_json", |b| {
        b.iter(|| {
            parse_from_str(black_box(&medium)).unwrap();
        })
    });

    c.bench_function("parse_large_json", |b| {
        b.iter(|| {
            parse_from_str(black_box(&large)).unwrap();
        })
    });

    c.bench_function("parse_empty_object", |b| {
        b.iter(|| {
            black_box(parse_from_str("{}").unwrap());
        })
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);