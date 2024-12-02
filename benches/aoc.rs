use advent_of_code::{day1, day2};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("AoC");
    group.sample_size(250);
    group.bench_function("day1", |b| b.iter(day1::day1));
    group.bench_function("day2", |b| b.iter(day2::day2));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
