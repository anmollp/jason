use std::collections::BTreeMap;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use jason::{diff, parse_from_str, JsonValue};

fn small_json_pair() -> (JsonValue, JsonValue) {
    let a = r#"
    {
        "users": [
            {"id":1,"name":"user1"},
            {"id":2,"name":"user2"},
            {"id":3,"name":"user3"}
        ]
    }
    "#;

    let b = r#"
    {
        "users": [
            {"id":1,"name":"user1"},
            {"id":2,"name":"changed"},
            {"id":3,"name":"user3"}
        ]
    }
    "#;

    (
        parse_from_str(a).unwrap(),
        parse_from_str(b).unwrap(),
    )
}

fn make_array_json(size: usize, changed_index: usize) -> (JsonValue, JsonValue) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 0..size {
        a.push(JsonValue::Number(i as f64));

        if i == changed_index {
            b.push(JsonValue::Number(999999.0));
        } else {
            b.push(JsonValue::Number(i as f64));
        }
    }

    (
        JsonValue::Array(a),
        JsonValue::Array(b),
    )
}

fn bench_diff(c: &mut Criterion) {
    let (old, new) = small_json_pair();

    c.bench_function("diff_small", |b| {
        b.iter(|| {
            black_box(diff(
                black_box(&old),
                black_box(&new),
            ));
        })
    });
}

fn bench_diff_scaling(c: &mut Criterion) {
    for size in [100, 1000, 10000] {
        let (a, b) = make_array_json(size, size / 2);

        c.bench_function(
            &format!("diff_array_{}", size),
            |bench| {
                bench.iter(|| {
                    black_box(diff(
                        black_box(&a),
                        black_box(&b),
                    ));
                })
            },
        );
    }
}

fn make_object_json(size: usize, changed_index: usize) -> (JsonValue, JsonValue) {
    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();

    for i in 0..size {
        a.insert(format!("key{}", i), JsonValue::Number(i as f64));

        if i == changed_index {
            b.insert(format!("key{}", i), JsonValue::Number(9999.0));
        } else {
            b.insert(format!("key{}", i), JsonValue::Number(i as f64));
        }
    }

    (
        JsonValue::Object(a),
        JsonValue::Object(b),
    )
}

fn bench_diff_objects_scaling(c: &mut Criterion) {
    for size in [100, 1000, 10000] {
        let (a, b) = make_object_json(size, size / 2);
        c. bench_function(
            &format!("diff_object_{}", size),
            |bench| {
                bench.iter(|| {
                    black_box(diff(
                        black_box(&a),
                        black_box(&b),
                    ));
                })
        });
    }
}

criterion_group!(benches, bench_diff, bench_diff_scaling, bench_diff_objects_scaling);
criterion_main!(benches);