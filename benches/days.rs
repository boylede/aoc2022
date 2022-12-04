use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    use aoc2022::days::day1::*;
    c.bench_function("day 01", |b| b.iter(|| run(black_box(INPUT))));
}

fn day2(c: &mut Criterion) {
    use aoc2022::days::day2::*;
    c.bench_function("day 02", |b| b.iter(|| run(black_box(INPUT))));
}

fn day3(c: &mut Criterion) {
    use aoc2022::days::day3::*;
    c.bench_function("day 03", |b| b.iter(|| run(black_box(INPUT))));
}

fn day4(c: &mut Criterion) {
    use aoc2022::days::day4::*;
    c.bench_function("day 04", |b| b.iter(|| run(black_box(INPUT))));
}

criterion_group!(benches, day1, day2, day3, day4);
criterion_main!(benches);
