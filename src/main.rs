mod common;
mod patterns;

use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use rayon::prelude::*;

use common::{Word, MatchData, Pattern};
use patterns::compute_pattern;

fn main() {
    let mut guesses = read_words("data/valid.txt");
    let mut solutions = read_words("data/solutions.txt");

    // Extend the list of valid guesses with the solutions
    guesses.extend(solutions.iter().copied());

    while solutions.len() > 1 {
        let guess = find_best_splitter(&guesses, &solutions);
        println!("{} | Remaining options: {}", guess, solutions.len());

        print!("Input pattern: ");
        stdout().flush().unwrap();
        let mut pat_str = String::new();
        stdin().read_line(&mut pat_str).unwrap();
        let pattern = Pattern::from_input(&pat_str);

        let match_data = MatchData::from_word_match(&guess, &pattern);
        solutions = solutions.into_par_iter().filter(|w| match_data.matches(w)).collect();
    }

    println!("Solution: {}", solutions[0]);
}

fn read_words(file: &str) -> Vec<Word> {
    let string = read_to_string(file).unwrap_or_else(|_| panic!("Couldn't read file: {}", file));
    string.lines().map(Word::from_str).collect()
}

fn find_best_splitter(guesses: &[Word], solutions: &[Word]) -> Word {
    *guesses.par_iter()
            .map(|word| (word, expected_entropy(word, solutions)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
}

fn expected_entropy(guess: &Word, solutions: &[Word]) -> f64 {
    let mut pattern_count = [0.0; 243];
    let n_sols = solutions.len() as f64;

    for sol in solutions {
        let idx = compute_pattern(guess, sol).to_index();
        pattern_count[idx] += 1.0;
    }

    -pattern_count.into_iter()
        .filter(|&x| x > 0.0)    
        .map(|count| {
            let p = count / n_sols;
            let e = p.log2();
            p * e
        }).sum::<f64>()
}