use std::ops::{Add, Mul, Div, Rem};

use num::traits::Pow;

pub const INPUTS: &[&str] = &[INPUT, INPUT_A];
pub const INPUT: &str = include_str!("../../input/day11.txt");
pub const INPUT_A: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";

#[inline]
pub fn run(input: &str) -> (String, String) {
    let monkeys: Vec<_> = input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            lines.next();
            let items: Vec<i64> = {
                let (_, items) = lines.next().unwrap().split_once(": ").unwrap();
                items
                    .split(", ")
                    .map(|item| -> i64 { item.parse::<i64>().unwrap() })
                    .collect()
            };
            let operation = lines
                .next()
                .map(|operation| {
                    let (_, math) = operation.split_once(" = ").unwrap();
                    if math.contains('+') {
                        let (_, amount) = math.split_once(" + ").unwrap();
                        Modifier::Add(amount.parse::<i64>().unwrap())
                    } else if math.contains('*') {
                        let (_, amount) = math.split_once(" * ").unwrap();
                        match amount.parse::<i64>().ok() {
                            Some(amount) => Modifier::Multiply(i64::from(amount)),
                            None => Modifier::Square,
                        }
                    } else {
                        panic!("I don't understand the math");
                    }
                })
                .unwrap();
            let test = lines
                .next()
                .map(|line| -> i64 {
                    let (_, amount) = line.split_once(" by ").unwrap();
                    amount.parse::<i64>().unwrap()
                })
                .unwrap();
            let if_true = lines
                .next()
                .map(|line| {
                    let (_, monkey) = line.split_once(" monkey ").unwrap();
                    monkey.parse::<usize>().unwrap()
                })
                .unwrap();
            let if_false = lines
                .next()
                .map(|line| {
                    let (_, monkey) = line.split_once(" monkey ").unwrap();
                    monkey.parse::<usize>().unwrap()
                })
                .unwrap();
            (items, operation, test, if_true, if_false, 0)
        })
        .collect();

    let modulator: i64 = monkeys.iter().map(|(_,_,test,_,_,_)|test).product();
    // println!("setting modulator to {modulator}");
    let parta: usize = {
        
        let mut monkey_scores: Vec<_> = (0..20)
            .fold(monkeys.clone(), |state, round| {
                monkey_toss(state, 3, modulator, round)
            })
            .iter()
            .map(|monkey| monkey.5)
            .collect();
        monkey_scores.sort();
        monkey_scores.reverse();
        monkey_scores
            .into_iter()
            .take(2)
            .reduce(|a, b| a * b)
            .unwrap()
    };
    let partb = {
        let mut monkey_scores: Vec<_> = (0..10_000)
            .fold(monkeys.clone(), |state, round| {
                monkey_toss(state, 1,modulator, round)
            })
            .iter()
            .map(|monkey| monkey.5)
            .collect();
        monkey_scores.sort();
        monkey_scores.reverse();
        monkey_scores
            .into_iter()
            .take(2)
            .reduce(|a, b| a * b)
            .unwrap()
    };
    (format!("{parta}"), format!("{partb}"))
}

#[derive(Debug, Clone)]
enum Modifier {
    Square,
    Multiply(i64),
    Add(i64),
}


impl Modifier {
    fn apply(&self, num: i64) -> i64 {
        use Modifier::*;
        // println!("{self:?} @ {num}");
        match self {
            Square => num.pow(2),
            Multiply(mul) => num * mul,
            Add(add) => num + add,
        }
    }
}

type Monkey = (Vec<i64>, Modifier, i64, usize, usize, usize);

fn monkey_toss(mut state: Vec<Monkey>, reduction: i64, modulator: i64, _round: usize) -> Vec<Monkey> {
    // println!("Round {round}");
    for i in 0..state.len() {
        // println!("Monkey {i}");
        let modifier = state[i].1.clone();
        let test = state[i].2.clone();
        let true_dest = state[i].3;
        let false_dest = state[i].4;
        let mut replacement = vec![];
        std::mem::swap(&mut state[i].0, &mut replacement);
        state[i].5 += replacement.len();
        for item in replacement.drain(..) {
            // println!("\tMonkey inspects an item with a worry level of {item}.");
            let anxiety = modifier.apply(item);
            // println!("\t\tWorry level is changed to {anxiety}");
            let anxiety = (anxiety / reduction) % modulator;
            // println!("\t\tWorry level is changed to {anxiety}");
            if anxiety % test == 0.into() {
                // println!("\t\tcurrent worry level is divisible by {test}");
                // println!("\t\tItem with worry level {anxiety} is thrown to monkey {true_dest}.");
                state[true_dest].0.push(anxiety);
            } else {
                // println!("\t\tcurrent worry level is not divisible by {test}");
                // println!("\t\tItem with worry level {anxiety} is thrown to monkey {false_dest}.");
                state[false_dest].0.push(anxiety);
            }
        }
    }
    // println!("Round: {round} -------------");
    for (i, monkey) in state.iter().enumerate() {
        // println!("Monkey {i}: {:?}", monkey.0);
    }
    state
}
