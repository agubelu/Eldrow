use crate::common::MatchInfo;
use crate::dataloader::DataLoader;
use crate::entropy::find_best_splitter;
use crate::input::{ask_for_pattern, print_in_green};

use rayon::prelude::*;

// Play an interactive guessing game with the user
pub fn interactive_play(lang: &str) {
    let (guesses, mut solutions, translator) = DataLoader::load_language(lang);
    let n_chars = translator.count();

    println!("Use your keyboard to input the pattern that you got for every suggested word.");
    println!("g: Green, y: Yellow, x: Gray");
    println!("Enter to submit, backspace to go back.");
    println!("-------------------------------------");

    // A flag to remember if we guessed the solution by chance before
    // we were done pruning the solutions space, to avoid printing
    // it twice at the end of the game
    let mut guessed_midway = false;

    // Keep guessing until we only have one possible solution or we guess the word
    while solutions.len() > 1 {
        let guess = if solutions.len() == 2 {
            // If there is only two possible solutions left, we use
            // one of them, since we'll be right 50% of the time and
            // we aren't worsening the worse case if we miss.
            solutions[0]
        } else {
            find_best_splitter(&guesses, &solutions)
        };

        let guess_string = guess.as_string(&translator);
        print!("{}", guess_string);
        let pattern = ask_for_pattern(&guess_string);

        // If we randomly guessed it, remember it and finish playing
        if pattern.is_solved() {
            guessed_midway = true;
            break;
        }

        let match_data = MatchInfo::from_word_match(&guess, &pattern, n_chars);
        solutions = solutions.into_par_iter().filter(|w| match_data.matches(w)).collect();
    }

    if solutions.is_empty() {
        println!("Oops, no solutions found... Check that the color patterns are correct and try again.")
    } else if !guessed_midway {
        print_in_green(&solutions[0].as_string(&translator));
    }
}