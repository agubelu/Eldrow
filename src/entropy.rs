use crate::common::Word;

use rayon::prelude::*;

// Finds the word that maximizes expected entropy
// between the given list of possible solutions
pub fn find_best_splitter(guesses: &[Word], solutions: &[Word]) -> Word {
    *guesses.par_iter()
            .map(|word| (word, expected_entropy(word, solutions)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
}

// Computes the expected entropy for a word given a list of solutions
fn expected_entropy(guess: &Word, solutions: &[Word]) -> f64 {
    let mut pattern_count = [0.0; 243];  // The total number of possible
    let n_sols = solutions.len() as f64; // color patterns is 243, or 3^5

    for sol in solutions {
        let idx = guess.compute_pattern(sol).to_index();
        pattern_count[idx] += 1.0;
    }

    -pattern_count.into_iter() // Flip the sign because logs of numbers < 1 are negative
        .filter(|&x| x > 0.0) // Avoid NaNs when computing log2
        .map(|count| {
            let p = count / n_sols;
            let e = p.log2();
            p * e
        }).sum::<f64>()
}