pub const INPUTS: &[&str] = &[INPUT, TEST_INPUT];
pub const INPUT: &str = include_str!("../../input/day2.txt");
const TEST_INPUT: &str = "A Y
B X
C Z";

/// my succinct version of this puzzle solution, which uses a precalculated score for each position
#[inline]
pub fn run(input: &str) -> (String, String) {
    let (parta, partb): (i32, i32) = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let a = chars.next().unwrap_or(' ');
            let _ = chars.next();
            let b = chars.next().unwrap_or(' ');
            match a {
                'A' => match b {
                    'X' => (4, 3),
                    'Y' => (8, 4),
                    'Z' => (3, 8),
                    _ => (0, 0),
                },
                'B' => match b {
                    'X' => (1, 1),
                    'Y' => (5, 5),
                    'Z' => (9, 9),
                    _ => (0, 0),
                },
                'C' => match b {
                    'X' => (7, 2),
                    'Y' => (2, 6),
                    'Z' => (6, 7),
                    _ => (0, 0),
                },
                _ => (0, 0),
            }
        })
        .reduce(|(aa, bb), (a, b)| (aa + a, bb + b))
        .unwrap_or((0, 0));

    (format!("{parta}"), format!("{partb}"))
}

/// my initial version, which is longer code but more readable
#[inline]
pub fn run_lengthy(input: &str) -> (String, String) {
    let parta: i32 = {
        input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(' ').unwrap();
                (Choice::parse(a), Choice::parse(b))
            })
            .map(|(a, b)| b.score() + Outcome::resolve(a, b).score())
            .sum()
    };

    let partb: i32 = {
        input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(' ').unwrap();
                (Choice::parse(a), Outcome::parse(b))
            })
            .map(|(opponent, outcome)| (Choice::pick(opponent, outcome), outcome))
            .map(|(choice, outcome)| choice.score() + outcome.score())
            .sum()
    };

    (format!("{parta}"), format!("{partb}"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    /// pick my choice based on known opponent choice and known outcome
    fn pick(opponent: Choice, outcome: Outcome) -> Choice {
        use Choice::*;
        use Outcome::*;
        match (opponent, outcome) {
            (c, Draw) => c,
            (Rock, Win) => Paper,
            (Rock, Loss) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Loss) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Loss) => Paper,
        }
    }
    /// determine the score of this choice
    fn score(&self) -> i32 {
        use Choice::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    /// parse from string, using a concrete method rather than FromStr trait
    /// to avoid introducing unwraps at callsite and extra type specifiers at callsite
    fn parse(input: &str) -> Choice {
        use Choice::*;
        match input {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Input was invalid for Choice: {input}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn resolve(a: Choice, b: Choice) -> Outcome {
        use Choice::*;
        use Outcome::*;
        match (a, b) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Loss,
            (Paper, Rock) => Loss,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Win,
            (Scissors, Rock) => Win,
            (Scissors, Paper) => Loss,
            (Scissors, Scissors) => Draw,
        }
    }
    fn score(&self) -> i32 {
        use Outcome::*;
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
    fn parse(input: &str) -> Outcome {
        use Outcome::*;
        match input {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Input was invalid for Outcome: {input}"),
        }
    }
}

/// a version of the solution that pre-calculates the score for each possible game.
/// the performance of this version is worse than just calculating it on the fly
/// likely due to string comparison / hashing.
mod pre_calculated {
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    const LOOKUP_TABLE: &[(&str, i32, i32)] = &[
        ("A X", 4, 3),
        ("A Y", 8, 4),
        ("A Z", 3, 8),
        ("B X", 1, 1),
        ("B Y", 5, 5),
        ("B Z", 9, 9),
        ("C X", 7, 2),
        ("C Y", 2, 6),
        ("C Z", 6, 7),
    ];

    lazy_static! {
        pub static ref LOOKUP_MAP: HashMap<String, (i32, i32)> = {
            LOOKUP_TABLE
                .into_iter()
                .map(|(k, a, b)| (k.to_string(), (*a, *b)))
                .collect()
        };
    }

    pub fn run(input: &str) -> (String, String) {
        let parta: i32 = input
            .lines()
            .filter_map(|line| LOOKUP_MAP.get(line))
            .map(|(a, _b)| a)
            .sum();
        let partb: i32 = input
            .lines()
            .filter_map(|line| LOOKUP_MAP.get(line))
            .map(|(_a, b)| b)
            .sum();
        (format!("{parta}"), format!("{partb}"))
    }

    pub fn build_lookup() {
        use super::*;
        for (pat, a, b) in LOOKUP_TABLE.iter() {
            let pa: i32 = pat
                .lines()
                .map(|line| {
                    let (a, b) = line.split_once(' ').unwrap();
                    (Choice::parse(a), Choice::parse(b))
                })
                .map(|(a, b)| b.score() + Outcome::resolve(a, b).score())
                .sum();

            let pb: i32 = pat
                .lines()
                .map(|line| {
                    let (a, b) = line.split_once(' ').unwrap();
                    (Choice::parse(a), Outcome::parse(b))
                })
                .map(|(opponent, outcome)| (Choice::pick(opponent, outcome), outcome))
                .map(|(choice, outcome)| choice.score() + outcome.score())
                .sum();
            assert!(pa == *a);
            assert!(pb == *b);
            println!("{pat} -> a: {pa}, b: {pb}")
        }
    }
}
