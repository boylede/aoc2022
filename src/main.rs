use aoc2022::LOOKUP_TABLE;
use clap::Parser;

fn main() {
    let Args { day, alt } = Args::parse();
    let Some((func, inputs)) = LOOKUP_TABLE.get(day) else {
        println!("Day {} not found", day);
        return;
    };
    let input_index = alt.unwrap_or(0);
    let Some(input) = inputs.get(input_index) else {
        println!("Input {} not found", input_index);
        return;
    };
    let (parta, partb) = func(input);
    println!("# Day {day}\n## Part A\n{parta}\n## Part B\n{partb}");
}

/// CLI runner harness for aoc entries
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[arg(short, long, default_value_t = 1)]
    day: usize,
    /// use an alternate input
    #[arg(short, long)]
    alt: Option<usize>,
}
