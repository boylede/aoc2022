pub const INPUTS: &[&str] = &[INPUT];
pub const INPUT: &str = include_str!("../../input/day4.txt");

pub fn run(input: &str) -> (String, String) {
    let ranges: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a_min, a_max) = a.split_once('-').unwrap();
            let (b_min, b_max) = b.split_once('-').unwrap();
            (
                a_min.parse::<i32>().unwrap(),
                a_max.parse::<i32>().unwrap(),
                b_min.parse::<i32>().unwrap(),
                b_max.parse::<i32>().unwrap(),
            )
        })
        .collect();

    let parta = ranges
        .iter()
        .filter(|(a_min, a_max, b_min, b_max)| {
            (a_min <= b_min && a_max >= b_max) || (b_min <= a_min && b_max >= a_max)
        })
        .count();

    let partb = ranges
        .iter()
        .filter(|(a_min, a_max, b_min, b_max)| {
            (a_min <= b_min && a_max >= b_min)
                || (a_min <= b_max && a_max >= b_max)
                || (b_min <= a_min && b_max >= a_min)
                || (b_min <= a_max && b_max >= a_max)
        })
        .count();
    (format!("{parta}"), format!("{partb}"))
}
