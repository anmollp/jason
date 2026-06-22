use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use jason::parse_from_str;

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

fn medium_json() -> String {
    let mut s = String::from("{\"users\":[");
    for i in 0..100 {
        s.push_str(&format!(
            "{{\"id\":{},\"name\":\"user{}\",\"active\":true}},",
            i, i
        ));
    }
    s.pop();
    s.push_str("]}");
    s
}

fn large_json() -> String {
    let mut s = String::from("{\"users\":[");
    for i in 0..10_000 {
        s.push_str(&format!(
            "{{\"id\":{},\"name\":\"user{}\",\"active\":true}},",
            i, i
        ));
    }
    s.pop();
    s.push_str("]}");
    s
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);