use crate::common::{Word, MatchInfo};
use crate::dataloader::{DataLoader, CharTranslator};
use crate::entropy::find_best_splitter;

use indicatif::ProgressIterator;
use rayon::prelude::*;

pub struct Benchmark {
    guesses: Vec<Word>,
    solutions: Vec<Word>,
    initial_word: Word,
    translator: CharTranslator
}

impl Benchmark {
    pub fn init(lang: &str) -> Self {
        let (guesses, solutions, translator) = DataLoader::load_language(lang);

        // The best initial word is always the same for a given
        // language, so we compute it during initialization
        let initial_word = find_best_splitter(&guesses, &solutions);
        Self { guesses, solutions, initial_word, translator }
    }

    pub fn run(&self) {
        println!("Running benchmark...");
        let mut counts = [0; 7];

        for solution in self.solutions.iter().progress() {
            let tries = self.play_round(*solution);
            counts[tries - 1] += 1;
        }

        let n_runs = self.solutions.len() as f32;
        let mut avg = 0.0;

        println!("Opening word: {}", self.initial_word.as_string(&self.translator));
        
        for (i, &count) in counts.iter().enumerate() {
            let s = if i == 6 { "X".to_owned() } else { (i+1).to_string() };
            let ratio = count as f32 / n_runs;
            avg += (i+1) as f32 * ratio;
            println!("- {}: {} ({:.2}%)", s, count, ratio * 100.0);
        }

        println!("Average: {:.4}", avg);
    }

    // Plays one round for a given solution and returns the amount
    // of tries it took to get to the solution
    fn play_round(&self, solution: Word) -> usize {
        let mut attempts = 1;
        let mut solutions = self.solutions.clone();
        let n_chars = self.translator.count();

        while attempts <= 6 {
            // Determine the word that we are going to try
            let guess = if solutions.len() <= 2 {
                // If there is only one or two possible solutions left
                // we use the first one
                // In the case of two solutions, the worst case is
                // already two turns, so by using one of them,
                // we'll get it right in one turn 50% of the time.
                solutions[0]
            } else if attempts == 1 {
                // If it's the first attempt, use the initial word
                self.initial_word
            } else {
                // Otherwise, determine the optimal word for the remaining
                // set of answers, updating it in the process
                find_best_splitter(&self.guesses, &solutions)
            };

            // If the guess is the solution, the game has finished
            if guess == solution {
                break;
            }

            // Otherwise, get the comparison pattern with the solution
            // and update the solutions list
            let pattern = guess.compute_pattern(&solution);
            let match_data = MatchInfo::from_word_match(&guess, &pattern, n_chars);
            solutions = solutions.into_par_iter().filter(|w| match_data.matches(w)).collect();
            attempts += 1;
        }

        attempts
    }
}