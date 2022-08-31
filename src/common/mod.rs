mod word;
mod pattern;
mod colors;
mod match_info;

// Re-export the main structs and functions
pub use word::Word;
pub use pattern::Pattern;
pub use colors::{Color, Colors};
pub use match_info::MatchInfo;