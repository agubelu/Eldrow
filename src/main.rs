mod benchmark;
mod common;
mod dataloader;
mod entropy;
mod input;
mod play;

use clap::Parser;

use benchmark::Benchmark;
use play::interactive_play;

#[derive(Parser)]
struct Args {
    #[clap(short, long, action)]
    benchmark: bool,

    #[clap(short, long, default_value = "en")]
    language: String,
}

fn main() {
    // Parse the command-line arguments
    let args = Args::parse();
    let lang = args.language;

    // If the benchmark flag is active, run it, otherwise play the game
    if args.benchmark {
        let bench = Benchmark::init(&lang);
        bench.run();
    } else {
        interactive_play(&lang);
    }
}
