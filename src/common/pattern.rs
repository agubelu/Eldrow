use crate::common::{Color, Colors};

pub struct Pattern { 
    pub colors: [Color; 5] 
}

impl Pattern {
    pub fn to_index(&self) -> usize {
        self.colors[4] as usize * 81 + // 3^4
        self.colors[3] as usize * 27 + // 3^3
        self.colors[2] as usize *  9 + // 3^2
        self.colors[1] as usize *  3 + // 3^1
        self.colors[0] as usize        // 3^0
    }

    pub fn is_solved(&self) -> bool {
        self.colors.iter().all(|&x| x == Colors::GREEN)
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self { colors: [Colors::GRAY; 5] }
    }
}