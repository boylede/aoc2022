use itertools::Itertools;

pub const INPUTS: &[&str] = &[INPUT];
pub const INPUT: &str = include_str!("../../input/day3.txt");

pub fn run(input: &str) -> (String, String) {
    let parta: i32 = input
        .lines()
        .map(|rucksack| {
            let len = rucksack.len();
            let (a, b) = rucksack.split_at(len / 2);
            let mut chars = a.chars();
            let item = loop {
                let Some(item) = chars.next() else {
                    panic!("ran out of items in line {rucksack}");
                };
                if b.contains(item) {
                    break item;
                }
            };
            item
        })
        .map(|item| priority(item))
        .sum();

    let partb: i32 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let leader = group.next().unwrap();
            let a = group.next().unwrap();
            let b = group.next().unwrap();
            for badge in leader.chars() {
                if a.contains(badge) && b.contains(badge) {
                    return badge;
                }
            }
            panic!("badge not found for group");
        })
        .map(|badge| priority(badge))
        .sum();
    (format!("{parta}"), format!("{partb}"))
}

fn priority(c: char) -> i32 {
    let c = c as i32;
    if c > 90 {
        c - 96
    } else {
        c - 64 + 26
    }
}

mod test {
    #[test]
    fn test_priority() {
        use super::priority;
        let a = priority('A');
        assert!(a == 27);
        let a = priority('a');
        assert!(a == 1);
    }
}
