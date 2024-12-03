use advent_of_code::{
    day1::{self, get_day_1_input},
    day2::{self, get_day_2_input},
    day3::{self, get_day_3_input},
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let day_1_input = get_day_1_input().unwrap();
    let day_2_input = get_day_2_input().unwrap();
    let day_3_input = get_day_3_input().unwrap();

    let mut group = c.benchmark_group("AoC");
    group.sample_size(100);

    group.bench_function("day1", |b| b.iter(|| day1::day1(day_1_input.clone())));
    group.bench_function("day2", |b| b.iter(|| day2::day2(day_2_input.clone())));
    group.bench_function("day3", |b| b.iter(|| day3::day3(day_3_input.clone())));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
