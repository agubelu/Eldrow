use crate::{SOLUTIONS_DATA, GUESSES_DATA, read_words};
use crate::common::{Word, MatchData};
use crate::patterns::compute_pattern;

use indicatif::ProgressIterator;
use rayon::prelude::*;

pub struct Benchmark {
    guesses: Vec<Word>,
    solutions: Vec<Word>,
    counts: [usize; 7],
    initial_word: Word
}

impl Benchmark {
    pub fn init() -> Self {
        // English only right now, in the future this should select
        // and load the word lists for the appropriate language
        let mut guesses = read_words(GUESSES_DATA);
        let solutions = read_words(SOLUTIONS_DATA);

        // Extend the list of valid guesses with the solutions
        guesses.extend(solutions.iter().copied());

        let counts = [0; 7];

        // The best initial word is always the same for a given
        // language, so we compute it during initialization
        let initial_word = Word::from_str("salet");
        // let initial_word = crate::find_best_splitter(&guesses, &solutions);
        Self { guesses, solutions, counts, initial_word }
    }

    pub fn run(&mut self) {
        println!("Running benchmark...");
        for solution in self.solutions.iter().progress() {
            let tries = self.play_round(*solution);
            self.counts[tries - 1] += 1;
        }

        let n_runs = self.solutions.len() as f32;
        let mut avg = 0.0;
        
        for i in 0..7 {
            let s = if i == 6 { "X".to_owned() } else { (i+1).to_string() };
            let count = self.counts[i];
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

        while attempts <= 6 {
            // Determine the word that we are going to try
            let guess = if solutions.len() <= 2 {
                // If there is only one or two possible solutions left
                // we use the first one
                // In the case of two solutions, the worst case is
                // already two turns, so by using one of them,
                // we'll save up a turn sometimes.
                solutions[0]
            } else if attempts == 1 {
                // If it's the first attempt, use the initial word
                self.initial_word
            } else {
                // Otherwise, determine the optimal word for the remaining
                // set of answers, updating it in the process
                crate::find_best_splitter(&self.guesses, &solutions)
            };

            // If the guess is the solution, the game has finished
            if guess == solution {
                break;
            }

            // Otherwise, get the comparison pattern with the solution
            // and update the solutions list
            let pattern = compute_pattern(&guess, &solution);
            let match_data = MatchData::from_word_match(&guess, &pattern);
            solutions = solutions.into_par_iter().filter(|w| match_data.matches(w)).collect();
            attempts += 1;
        }

        attempts
    }
}