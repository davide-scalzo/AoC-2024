use advent_of_code::day1::day1;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day1");
    group.sample_size(500);
    group.bench_function("day1", |b| b.iter(day1));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
