use clap::Parser;

use aoc2022::{days::*, LOOKUP_TABLE};

fn main() {
    let args = Args::parse();

    let Some((runner, input)) = LOOKUP_TABLE.get(args.day) else {
        println!("Day {} not found", args.day);
        return;
    };
    let results = runner(input);
    println!("{}: parta: {}, partb: {}", day1::LABEL, results.0, results.1);
}

/// CLI runner harness arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Which day to run
   #[arg(short, long, default_value_t = 1)]
   day: usize,
}
