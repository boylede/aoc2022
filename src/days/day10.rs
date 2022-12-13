use itertools::Itertools;

pub const INPUTS: &[&str] = &[INPUT, INPUT_A, INPUT_B];
pub const INPUT: &str = include_str!("../../input/day10.txt");

pub const BREAKPOINTS: &[usize] = &[20, 60, 100, 140, 180, 220];
pub const SCREEN_PIXELS: &[u8; 40] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];
#[inline]
pub fn run(input: &str) -> (String, String) {
    let program: Vec<_> = input
        .lines()
        .flat_map(|line| {
            if let Some((_, b)) = line.split_once(' ') {
                vec![Instruction::Noop, Instruction::Addx(b.parse().unwrap())]
            } else {
                vec![Instruction::Noop]
            }
        })
        .collect();
    let parta: i32 = program
        .iter()
        .scan((0usize, 1i32), |(tick, x), instruction| {
            let ox: i32 = *x;
            match instruction {
                Instruction::Noop => (),
                Instruction::Addx(dx) => *x += dx,
            }
            *tick += 1;
            Some((*tick, ox))
        })
        .filter(|(tick, _x)| BREAKPOINTS.contains(tick))
        // .inspect(|(tick, x)| println!("during {tick} x={x}, score is {}", *tick as i32 * *x))
        .map(|(tick, x)| tick as i32 * x)
        .sum();

    let screen = program
        .iter()
        .scan((0usize, 1i32), |(tick, x), instruction| {
            let ox: i32 = *x;
            match instruction {
                Instruction::Noop => (),
                Instruction::Addx(dx) => *x += dx,
            }
            *tick += 1;
            Some((*tick, ox))
        })
        .fold(
            [
                SCREEN_PIXELS.to_owned(),
                SCREEN_PIXELS.to_owned(),
                SCREEN_PIXELS.to_owned(),
                SCREEN_PIXELS.to_owned(),
                SCREEN_PIXELS.to_owned(),
                SCREEN_PIXELS.to_owned(),
            ],
            |mut screen, (tick, x)| {
                let col = (tick % 40).saturating_sub(1);
                let row = (tick / 40) % 6;
                let lit = (x - 1 <= col as i32) && (x + 1 >= col as i32);
                let Some(pixel) = screen[row].get_mut(col) else {
                    panic!("unexpected index");
                };
                *pixel = if lit { 1 } else { 0 };
                screen
            },
        );
    let partb = screen
        .iter()
        .map(|row| {
            row.iter()
                .map(|p| if *p >= 1 { '#' } else { ' ' })
                .collect::<String>()
        })
        .join("\n");
    (format!("{parta}"), format!("{partb}"))
}

pub enum Instruction {
    Noop,
    Addx(i32),
}

pub const INPUT_A: &str = "noop
addx 3
addx -5";

pub const INPUT_B: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
