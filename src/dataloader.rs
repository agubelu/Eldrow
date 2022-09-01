use std::collections::hash_map::Entry;
use std::fs::read_to_string;
use rustc_hash::FxHashMap;

use crate::common::Word;

pub struct DataLoader;

// When loading a language's data, we transform the characters in the word
// to numbers, starting from 0 and increasing for every new character that
// is found. This allows the word's characters to be used as indices for
// the arrays that are sometimes used for efficiency and normalizes all
// alphabets in the [0-N) range instead of relying on the encoding used
// by char, which can contain large gaps.
// This struct keeps track of the mapping from characters to indices, to
// be able to "translate" words back to be displayed to the user.
pub struct CharTranslator {
    char_to_index: FxHashMap<char, u16>,
    index_to_char: Vec<char>
}

impl DataLoader {
    pub fn load_language(lang: &str) -> (Vec<Word>, Vec<Word>, CharTranslator) {
        let mut translator = CharTranslator::new();

        // Load the text files from the language's folder, panicking with
        // an error immediately if they don't exist
        let path_valid_words = format!("data/{}/valid.txt", lang.to_lowercase());
        let path_solutions = format!("data/{}/solutions.txt", lang.to_lowercase());

        // Make sure to turn all the words to lowercase for consistency
        let valid_words_str = read_to_string(path_valid_words)
            .expect("Could not find the list of valid words for the specified language")
            .to_lowercase();
        let solutions_str = read_to_string(path_solutions)
            .expect("Could not find the list of solutions for the specified language")
            .to_lowercase();

        // Update the character translator with the words found in both files
        translator.update(&valid_words_str);
        translator.update(&solutions_str);

        // Create the lists of valid guesses and solutions
        let mut valid_words = read_words(&valid_words_str, &translator);
        let solutions = read_words(&solutions_str, &translator);

        // Extend the list of valid guesses with the solutions
        valid_words.extend(solutions.iter().copied());

        // Make sure that the list of valid words doesn't contain duplicates.
        // In some wordles, they sometimes also contain the solutions, so
        // the previous step may have duplicated them
        valid_words.sort_unstable();
        valid_words.dedup();

        (valid_words, solutions, translator)
    }
}

impl CharTranslator {
    pub fn new() -> Self {
        let char_to_index = FxHashMap::default();
        let index_to_char = vec![];
        Self { char_to_index, index_to_char }
    }

    // Updates the translator with the contents of a file,
    // which are one word per line. Each word is expected to have
    // exactly five characters.
    pub fn update(&mut self, file_content: &str) {
        for line in file_content.lines() {
            let chars: Vec<char> = line.chars().collect();
            assert_eq!(chars.len(), 5, "The following word does not have 5 characters: {}", line);
            for ch in chars {
                if let Entry::Vacant(e) = self.char_to_index.entry(ch) {
                    // The index for this character will be the list's current
                    // length, which is an index that has not been assigned yet
                    let idx = self.index_to_char.len() as u16;
                    e.insert(idx);
                    self.index_to_char.push(ch);
                }
            }
        }
    }

    // Gets the character for an index, assuming it exists
    pub fn index_to_char(&self, idx: u16) -> char {
        self.index_to_char[idx as usize]
    }

    // Gets the index for a character, assuming it exists
    pub fn char_to_index(&self, ch: char) -> u16 {
        self.char_to_index[&ch]
    }

    pub fn count(&self) -> usize {
        self.char_to_index.len()
    }
}

pub fn read_words(string: &str, translator: &CharTranslator) -> Vec<Word> {
    string.lines().map(|line| Word::from_str(line, translator)).collect()
}