mod benchmark;
mod common;
mod input;
mod patterns;

use rayon::prelude::*;

use common::{Word, MatchData};
use patterns::compute_pattern;

pub static GUESSES_DATA: &str = include_str!("../data/valid.txt");
pub static SOLUTIONS_DATA: &str = include_str!("../data/solutions.txt");

fn main() {
    let mut guesses = read_words(GUESSES_DATA);
    let mut solutions = read_words(SOLUTIONS_DATA);
  
    println!("Use your keyboard to input the pattern that you got for every suggested word.");
    println!("g: Green, y: Yellow, x: Gray");
    println!("Enter to submit, backspace to go back.");
    println!("-------------------------------------");

    // Extend the list of valid guesses with the solutions
    guesses.extend(solutions.iter().copied());

    while solutions.len() > 1 {
        let guess = if solutions.len() == 2 {
            solutions[0]
        } else {
            find_best_splitter(&guesses, &solutions)
        };
        print!("{}", guess);
        let pattern = input::ask_for_pattern(&guess);
        let match_data = MatchData::from_word_match(&guess, &pattern);
        solutions = solutions.into_par_iter().filter(|w| match_data.matches(w)).collect();
    }

    if solutions.is_empty() {
        println!("Oops, no solutions found... Check that the color patterns are correct and try again.")
    } else {
        input::print_in_green(&solutions[0]);
    }
}

pub fn read_words(string: &str) -> Vec<Word> {
    //let string = read_to_string(file).unwrap_or_else(|_| panic!("Couldn't read file: {}", file));
    string.lines().map(Word::from_str).collect()
}

// Returns the best splitter word for the given list of
// solutions, along with the expected entropy.
pub fn find_best_splitter(guesses: &[Word], solutions: &[Word]) -> Word {
    *guesses.par_iter()
            .map(|word| (word, expected_entropy(word, solutions)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
}

// Computes the expected entropy for a word given a list of solutions
fn expected_entropy(guess: &Word, solutions: &[Word]) -> f64 {
    let mut pattern_count = [0.0; 243];
    let n_sols = solutions.len() as f64;

    for sol in solutions {
        let idx = compute_pattern(guess, sol).to_index();
        pattern_count[idx] += 1.0;
    }

    -pattern_count.into_iter()
        .filter(|&x| x > 0.0) // Avoid NaNs when computing log2
        .map(|count| {
            let p = count / n_sols;
            let e = p.log2();
            p * e
        }).sum::<f64>()
}