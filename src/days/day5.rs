use std::str::FromStr;

pub const INPUTS: &[&str] = &[INPUT];
pub const INPUT: &str = include_str!("../../input/day5.txt");

pub fn run(input: &str) -> (String, String) {
    let lines: Vec<&str> = input.lines().collect();
    let (index_line, indexes) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| {
            line.chars()
                .all(|c| c.is_ascii_digit() || c.is_whitespace())
        })
        .unwrap();
    let collection: Vec<Vec<char>> = {
        let mut collection: Vec<Vec<char>> = indexes
            .split_ascii_whitespace()
            .map(|_| Vec::with_capacity(index_line))
            .collect();
        for i in 0..index_line {
            let i = index_line - i - 1;
            let line = lines.get(i).unwrap();
            for (j, set) in collection.iter_mut().enumerate() {
                let position = j * 4 + 1;
                if let Some(c) = line.get(position..position + 1) {
                    if let Some(c) = c.chars().next().filter(|c| c.is_ascii_alphabetic()) {
                        set.push(c);
                    }
                }
            }
        }
        collection
    };

    let mut collection_a = collection.clone();

    let instructions: Vec<(usize, usize, usize)> = lines
        .iter()
        .skip(index_line + 2)
        .map(|instruction| {

            let mut tokens = instruction.split_whitespace();
            tokens.next();
            let repeat = tokens
                .next()
                .map(<usize as FromStr>::from_str)
                .map(Result::unwrap)
                .unwrap();
            tokens.next();
            let from = tokens
                .next()
                .map(<usize as FromStr>::from_str)
                .map(Result::unwrap)
                .unwrap();
            tokens.next();
            let to = tokens
                .next()
                .map(<usize as FromStr>::from_str)
                .map(Result::unwrap)
                .unwrap();
            (repeat, from - 1, to - 1)
        })
        .collect();

    for (repeat, from, to) in instructions.clone() {

        for _ in 0..repeat {
            if to > from {

                let (from_half, to_half) = collection_a.split_at_mut(to);
                let to = to_half.get_mut(0).unwrap();
                let from = from_half.get_mut(from).unwrap();
                to.push(from.pop().unwrap())
            } else {

                let (to_half, from_half) = collection_a.split_at_mut(from);

                let to = to_half.get_mut(to).unwrap();
                let from = from_half.get_mut(0).unwrap();
                to.push(from.pop().unwrap())
            }
        }
    }
    let parta: String = collection_a
        .into_iter()
        .map(|mut set| set.pop().unwrap())
        .collect();

    let mut collection_b = collection.clone();

    for (repeat, from, to) in instructions {


        if to > from {

            let (from_half, to_half) = collection_b.split_at_mut(to);
            let to = to_half.get_mut(0).unwrap();
            let from = from_half.get_mut(from).unwrap();
            to.append(&mut from.split_off(from.len() - repeat))
        } else {

            let (to_half, from_half) = collection_b.split_at_mut(from);

            let to = to_half.get_mut(to).unwrap();
            let from = from_half.get_mut(0).unwrap();
            to.append(&mut from.split_off(from.len() - repeat))
        }
    }

    let partb: String = collection_b
        .into_iter()
        .map(|mut set| set.pop().unwrap())
        .collect();

    (parta, partb)
}
