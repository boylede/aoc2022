use std::collections::HashSet;

pub const INPUTS: &[&str] = &[INPUT, INPUT_A];
pub const INPUT: &str = include_str!("../../input/day9.txt");

pub const INPUT_A: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[inline]
pub fn run(input: &str) -> (String, String) {
    let mut positions = input
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let count = b.parse::<usize>().unwrap();
            match a {
                "U" => vec![(0, 1); count],
                "D" => vec![(0, -1); count],
                "L" => vec![(-1, 0); count],
                "R" => vec![(1, 0); count],
                _ => unimplemented!(),
            }
        })
        .scan(((0, 0), (0, 0)), |(head, tail), motion| {
            head.0 += motion.0;
            head.1 += motion.1;
            // println!("moved head to {head:?}");

            let w: i32 = head.0 - tail.0;
            let h: i32 = head.1 - tail.1;
            if w.abs() > 1 {
                tail.0 += w.signum();
                tail.1 += h.signum();
                
            } else if h.abs() > 1 {
                tail.0 += w.signum();
                tail.1 += h.signum();
            }
            // println!("tail is at {tail:?}");
            Some((tail.0, tail.1))
        })
        .collect::<HashSet<_>>();
    positions.insert((0, 0));

    let parta = positions.len();
    let partb = 0;
    (format!("{parta}"), format!("{partb}"))
}
