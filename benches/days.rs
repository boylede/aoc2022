use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1_benchmark(c: &mut Criterion) {
    use aoc2022::days::day1::*;
    c.bench_function("day 01", |b| b.iter(|| run(black_box(INPUT))));
}

criterion_group!(benches, day1_benchmark);
criterion_main!(benches);
