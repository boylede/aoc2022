use clap::Parser;

use aoc2022::LOOKUP_TABLE;

fn main() {
    let Args {day} = Args::parse();

    let Some((runner, input)) = LOOKUP_TABLE.get(day) else {
        println!("Day {} not found", day);
        return;
    };
    let results = runner(input);
    println!(
        "Day {}: parta: {}, partb: {}",
        day,
        results.0,
        results.1
    );
}

/// CLI runner harness for aoc entries
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[arg(short, long, default_value_t = 1)]
    day: usize,
}
