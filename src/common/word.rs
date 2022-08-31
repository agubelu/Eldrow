use crate::common::{Pattern, Colors};
use crate::dataloader::CharTranslator;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Word {
    pub chars: [u16; 5], 
}

impl Word {
    pub fn from_str(string: &str, translator: &CharTranslator) -> Self {
        let vu: Vec<u16> = string.chars()
            .map(|ch| translator.char_to_index(ch))
            .collect();
        let chars = [vu[0], vu[1], vu[2], vu[3], vu[4]];
        Self { chars }
    }

    pub fn as_string(&self, translator: &CharTranslator) -> String {
        self.chars.iter()
            .map(|&idx| translator.index_to_char(idx).to_ascii_uppercase())
            .collect()
    }

    // Computes the color pattern that you would get if you used
    // this word against the provided solution
    // We also have to know the total number of possible characters,
    // to construct the count list with the appropriate size
    pub fn compute_pattern(&self, solution: &Word, n_chars: usize) -> Pattern {
        let mut counts = vec![0; n_chars];
        let mut pattern = Pattern::default();

        // Initialize the letter counter
        solution.chars.iter().for_each(|&ch| {
            counts[ch as usize] += 1;
        });

        // Look for exact (green) matches first
        (0..5).for_each(|i| {
            if self.chars[i] == solution.chars[i] {
                let idx = self.chars[i] as usize;
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
                let ch = self.chars[i];
                let idx = ch as usize;
                if letter_in_word(ch, solution) && counts[idx] > 0 {
                    pattern.colors[i] = Colors::YELLOW;
                    counts[idx] -= 1;
                }
            }
        });

        pattern
    }
}

// Aux function to determine if a word contains a letter in any position
fn letter_in_word(letter: u16, word: &Word) -> bool {
    word.chars.iter().any(|ch| *ch == letter)
}