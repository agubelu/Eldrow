use crate::common::{Pattern, Colors};

use std::io::Write;
use std::process::exit;
use crossterm::{cursor, execute, terminal};
use crossterm::event::{Event, KeyEvent, read, KeyModifiers, KeyCode};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Avoid clashes with termcolor::Color
type WordleColor = crate::common::Color;

// Interactively asks the user for the color pattern, updating the
// current line to reflect the color selections made by the user
pub fn ask_for_pattern(word: &str) -> Pattern {
    // Enter raw mode to capture the inputs
    terminal::enable_raw_mode()
        .expect("Your console is not compatible with raw mode, which is required to run Eldrow.");
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut pos = 0;
    let mut pattern = Pattern::default();
    let mut done = false;
    let mut ctrl_c = false;
    let chars: Vec<char> = word.chars().collect();

    // Set the cursor to the beggining of the line
    execute!(stdout, cursor::MoveToColumn(0), cursor::Hide, cursor::DisableBlinking).unwrap();
    
    while !done {
        // Read the next key event
        match &read_key_blocking() {
            // We're in raw mode so we must take care of processing CTRL+C ourselves
            ev if is_ctrl_c(ev) => {
                ctrl_c = true;
                break;
            },
            // In any other case, process the input:
            KeyEvent { code, modifiers: _, .. } => match code {
                // Process enter if we are done with the pattern
                KeyCode::Enter if pos == 5 => done = true,
                // Process backspace if the pattern isn't empty
                KeyCode::Backspace if pos != 0 => {
                    pos -= 1;
                    execute!(stdout, cursor::MoveLeft(1)).unwrap();
                    write!(&mut stdout, "{}", chars[pos]).unwrap();
                    stdout.flush().unwrap();
                    execute!(stdout, cursor::MoveLeft(1)).unwrap();
                }
                // Process any other keycode if the pattern isn't full
                KeyCode::Char('x') | KeyCode::Char('y') | KeyCode::Char('g') if pos < 5 => {
                    // Color the current character
                    let (color_spec, wordle_color) = get_color_bg(code);

                    // Update the pattern
                    pattern.colors[pos] = wordle_color;

                    // Print the current character in the correct background color
                    stdout.set_color(&color_spec).unwrap();
                    write!(&mut stdout, "{}", chars[pos]).unwrap();
                    stdout.flush().unwrap();
                    pos += 1;

                    // Reset the color spec
                    stdout.reset().unwrap();
                }
                
                _ => {}
            }
        };
    }

    execute!(stdout, cursor::Show, cursor::EnableBlinking).unwrap(); // Restore to previous state
    terminal::disable_raw_mode().unwrap();  // If we enabled it we should be able to disable it, right...?

    // Print a newline in preparation for the next word
    println!();

    // If we reached here because of a CTRL+C, end the process
    if ctrl_c {
        exit(0);
    }

    pattern
}

// Prints the final solution with a green background,
// resetting stdout color afterwards before exiting
pub fn print_in_green(word: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Ansi256(71))).set_fg(Some(Color::Black))).unwrap();
    write!(stdout, "{}", word).unwrap();
    stdout.reset().unwrap();
    writeln!(stdout).unwrap();
    stdout.flush().unwrap();
}

// Pretty self-explanatory
fn read_key_blocking() -> KeyEvent {
    loop {
        if let Event::Key(ev) = read().unwrap() {
            return ev;
        }
    }
}

// Gets the correct background color for a keypress
fn get_color_bg(code: &KeyCode) -> (ColorSpec, WordleColor) {
    let (spec_color, wordle_color) = match code {
        KeyCode::Char('x') => (Color::Ansi256(242), Colors::GRAY),
        KeyCode::Char('y') => (Color::Ansi256(178), Colors::YELLOW),
        KeyCode::Char('g') => (Color::Ansi256(71), Colors::GREEN),
        _ => unreachable!(),
    };

    let mut spec = ColorSpec::new();
    spec.set_bg(Some(spec_color)).set_fg(Some(Color::Black));
    (spec, wordle_color)
}

fn is_ctrl_c(ev: &KeyEvent) -> bool {
    matches!(ev, KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. })
}