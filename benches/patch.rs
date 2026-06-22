use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
mod common;
use common::{large_document, make_replace_operations, make_remove_operations};

fn bench_patch_scaling(c: &mut Criterion) {
    let original = large_document();

    for size in [100, 1000, 10000] {
        let operations = make_replace_operations(size);
        c.bench_function(&format!("patch_replace_{}", size), |b| {
            b.iter_batched(|| original.clone(), |mut doc| {
                for op in &operations {
                    black_box(doc.apply(op.clone()).unwrap());
                }
                black_box(doc);
            },
            BatchSize::SmallInput)
        });
    }

    for size in [1000, 10000, 50000] {
        let operations = make_remove_operations(size);
        c.bench_function(&format!("patch_remove_{}", size), |b| {
            b.iter_batched(
                || original.clone(),
                |mut doc| {
                    for op in &operations {
                        black_box(doc.apply(op.clone()).unwrap());
                    }
                    black_box(doc);
                },
                BatchSize::SmallInput,
            );
        });
    }
}

criterion_group!(benches, bench_patch_scaling);
criterion_main!(benches);
