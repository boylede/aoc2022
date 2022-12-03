use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    use aoc2022::days::day1::*;
    c.bench_function("day 01", |b| b.iter(|| run(black_box(INPUT))));
}

fn day2(c: &mut Criterion) {
    use aoc2022::days::day2::*;
    c.bench_function("day 02", |b| b.iter(|| run(black_box(INPUT))));
}

criterion_group!(benches, day1, day2);
criterion_main!(benches);
