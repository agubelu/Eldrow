use std::fmt::{Display, Formatter, Result, Debug};

use crate::common::{Word, Pattern, Colors, MAT_WIDTH, chtoi};

pub fn compute_pattern(guess: &Word, solution: &Word) -> Pattern {
    let mut counts = [0; MAT_WIDTH];

    let mut pattern = Pattern::new();

    // Initialize the letter counter
    solution.chars.iter().for_each(|ch| {
        let idx = chtoi(*ch);
        counts[idx] += 1;
    });

    // Look for exact (green) matches first
    (0..5).for_each(|i| {
        if guess.chars[i] == solution.chars[i] {
            let idx = chtoi(guess.chars[i]);
            counts[idx] -= 1;
            pattern.colors[i] = Colors::GREEN;
        }
    });

    // Now look for yellow matches
    (0..5).for_each(|i| {
        // Add a yellow match if the current position isn't green,
        // and the current letter is in the solution, and we
        // haven't matched all instances of that letter yet
        if pattern.colors[i] != Colors::GREEN {
            let ch = guess.chars[i];
            let idx = chtoi(ch);
            if letter_in_word(ch, &solution) && counts[idx] > 0 {
                pattern.colors[i] = Colors::YELLOW;
                counts[idx] -= 1;
            }
        }
    });

    pattern
}

// Aux function to determine if a word contains a letter in any position
fn letter_in_word(letter: char, word: &Word) -> bool {
    word.chars.iter().any(|ch| *ch == letter)
}

////// Trait implementations
impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&String::from_iter(&self.chars).to_uppercase())
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&String::from_iter(&self.chars).to_uppercase())
    }
}