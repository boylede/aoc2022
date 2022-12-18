use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! day {
    ($num:expr, $name:ident, $path:path) => {
        fn $name(c: &mut Criterion) {
            use $path::*;
            c.bench_function($num, |b| b.iter(|| run(black_box(INPUT))));
        }
    };
}

day!("day 1", day1, ::aoc2022::days::day1);
day!("day 2", day2, ::aoc2022::days::day2);
day!("day 3", day3, ::aoc2022::days::day3);
day!("day 4", day4, ::aoc2022::days::day4);
day!("day 5", day5, ::aoc2022::days::day5);
day!("day 6", day6, ::aoc2022::days::day6);
day!("day 7", day7, ::aoc2022::days::day7);
day!("day 8", day8, ::aoc2022::days::day8);
day!("day 9", day9, ::aoc2022::days::day9);
day!("day 10", day10, ::aoc2022::days::day10);
day!("day 11", day11, ::aoc2022::days::day11);
day!("day 12", day12, ::aoc2022::days::day12);

criterion_group!(
    benches, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12
);
criterion_main!(benches);
