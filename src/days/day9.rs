use std::collections::HashSet;

use itertools::Itertools;

pub const INPUTS: &[&str] = &[INPUT, INPUT_A, INPUT_B];
pub const INPUT: &str = include_str!("../../input/day9.txt");

pub const INPUT_A: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

pub const INPUT_B: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[inline]
pub fn run(input: &str) -> (String, String) {
    let route: Vec<_> = input
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
        .collect();
    let parta = {
        let mut positions = route
            .iter()
            .scan(((0, 0), (0, 0)), |(head, tail), motion| {
                head.0 += motion.0;
                head.1 += motion.1;
                update_tail(head,tail);
                Some((tail.0, tail.1))
            })
            .collect::<HashSet<_>>();
        positions.insert((0, 0));
        positions.len()
    };

    let partb = {
        let mut positions = route
            .iter()
            .scan([(0, 0); 10], |state, motion| {
                state[0].0 += motion.0;
                state[0].1 += motion.1;
                for i in 0..(state.len()-1) {
                    let head = state[i];
                    let tail = &mut state[i+1];
                    update_tail(&head,tail);
                }
                Some(state[9])
            })
            .collect::<HashSet<_>>();

        positions.insert((0, 0));
        positions.len()
    };
    (format!("{parta}"), format!("{partb}"))
}

fn update_tail(head: &(i32, i32), tail: &mut (i32,i32)) {
    let w: i32 = head.0 - tail.0;
                let h: i32 = head.1 - tail.1;
                if w.abs() > 1 {
                    tail.0 += w.signum();
                    tail.1 += h.signum();
                } else if h.abs() > 1 {
                    tail.0 += w.signum();
                    tail.1 += h.signum();
                }
}
