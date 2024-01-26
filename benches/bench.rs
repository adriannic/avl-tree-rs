use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use avl_tree::{AVLTree};

fn criterion_benchmark(c: &mut Criterion) {
    let mut tree = AVLTree::new(0);
    
    let mut group = c.benchmark_group("benchmark");
        for i in [50, 500, 5000].iter() {
            group.bench_function(BenchmarkId::new("bench", i), |b| b.iter(|| tree.add_value(*i)));
    }
    group.finish();
}

criterion_group!(benchmarks, criterion_benchmark);
criterion_main!(benchmarks);