use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub const INPUTS: &[&str] = &[INPUT, INPUT_A, INPUT_B];
pub const INPUT: &str = include_str!("../../input/day6.txt");

const INPUT_A: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const INPUT_B: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

#[inline]
pub fn run(input: &str) -> (String, String) {
    let mut first = input.chars().enumerate();
    let (_, a) = first.next().unwrap();
    let (_, b) = first.next().unwrap();
    let (_, c) = first.next().unwrap();
    let (_, _, _, parta) = first
        .fold_while((a, b, c, 0), |(a, b, c, _), (index, d)| {
            if [b, c, d].contains(&a) || [c, d].contains(&b) || c == d {
                Continue((b, c, d, index + 1))
            } else {
                Done((b, c, d, index + 1))
            }
        })
        .into_inner();
    let chunks: Vec<_> = input.chars().collect();
    let partb = chunks
        .windows(14)
        .enumerate()
        .find(|(_, chunk)| all_unique(&chunk))
        .map(|(i, _)| i + 14)
        .unwrap();
    (format!("{parta}"), format!("{partb}"))
}

fn all_unique(input: &[char]) -> bool {
    for i in 0..input.len() {
        let c = input.get(i).unwrap();
        if input.iter().skip(i + 1).any(|cc| c == cc) {
            return false;
        } else {
            continue;
        }
    }
    return true;
}
