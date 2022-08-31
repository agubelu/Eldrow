mod benchmark;
mod common;
mod entropy;
mod input;
mod play;

use common::Word;
use benchmark::Benchmark;
use play::interactive_play;

use clap::Parser;

pub static GUESSES_DATA: &str = include_str!("../data/valid.txt");
pub static SOLUTIONS_DATA: &str = include_str!("../data/solutions.txt");

#[derive(Parser)]
struct Args {
    #[clap(short, long, action)]
    benchmark: bool,
}

fn main() {
    // Parse the command-line arguments
    let args = Args::parse();

    // If the benchmark flag is active, run it, otherwise play the game
    if args.benchmark {
        let bench = Benchmark::init();
        bench.run();
    } else {
        interactive_play();
    }
}

pub fn read_words(string: &str) -> Vec<Word> {
    string.lines().map(Word::from_str).collect()
}

