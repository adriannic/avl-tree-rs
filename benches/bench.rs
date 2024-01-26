use avl_tree::AVLTree;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark");
    for i in [50, 500, 5000].iter() {
        group.bench_function(BenchmarkId::new("bench", i), |b| {
            b.iter_batched(
                || AVLTree::new(0),
                |mut tree| {
                    for j in 1..*i {
                        tree.add_value(j);
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benchmarks, criterion_benchmark);
criterion_main!(benchmarks);
