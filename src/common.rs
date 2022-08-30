pub const MAT_WIDTH: usize = 'z' as usize - 'a' as usize + 1;
pub type Color = u8;
pub type MatrixCell = u8;

pub struct Colors;
impl Colors {
    pub const GRAY: Color = 0;
    pub const YELLOW: Color = 1;
    pub const GREEN: Color = 2;

    pub fn from_char(ch: char) -> Color {
        match ch {
            'x' | 'X' => Colors::GRAY,
            'y' | 'Y' => Colors::YELLOW,
            'g' | 'G' => Colors::GREEN,
            _ => unreachable!()
        }
    }
}

pub struct MatrixData;
impl MatrixData {
    pub const UNKNOWN: MatrixCell = 0;
    pub const NO: MatrixCell = 1;
    pub const EXACT: MatrixCell = 2;
}

pub struct Pattern { 
    pub colors: [Color; 5] 
}
impl Pattern {
    pub fn new() -> Self {
        Self { colors: [Colors::GRAY; 5] }
    }

    pub fn from_input(data: &str) -> Self {
        let input = data.trim();
        assert_eq!(input.len(), 5);
        let mut pattern = Self::new();

        input.chars().enumerate().for_each(|(i, ch)| {
            pattern.colors[i] = Colors::from_char(ch);
        });

        pattern
    }

    pub fn to_index(&self) -> usize {
        self.colors[4] as usize * 81 + // 3^4
        self.colors[3] as usize * 27 + // 3^3
        self.colors[2] as usize *  9 + // 3^2
        self.colors[1] as usize *  3 + // 3^1
        self.colors[0] as usize        // 3^0
    }

    pub fn to_string(&self) -> String {
        self.colors.iter()
            .map(|&x| match x {
                Colors::GRAY => 'X',
                Colors::YELLOW => 'C',
                Colors::GREEN => 'O',
                _ => 'L',
            })
            .collect()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Word {
     pub chars: [char; 5] 
}
impl Word {
    pub fn from_str(string: &str) -> Self {
        assert_eq!(string.len(), 5);
        let cv: Vec<char> = string.chars().collect();
        let chars = [cv[0], cv[1], cv[2], cv[3], cv[4]];
        Self { chars }
    }
}

// Auxiliary data used to quickly filter out words
// that don't match a given pattern
pub struct MatchData {
    pub matrix: [[MatrixCell; 5]; MAT_WIDTH],
    pub counters: [u8; MAT_WIDTH],
    pub yellow_chars: Vec<usize>
}
impl MatchData {
    pub fn from_word_match(word: &Word, pattern: &Pattern) -> Self {
        let matrix = [[MatrixData::UNKNOWN; 5]; MAT_WIDTH];
        let counters = [0; MAT_WIDTH];
        let yellow_chars = Vec::with_capacity(5);

        // Initialization
        let mut data = Self { matrix, counters, yellow_chars };
        for (i, (&ch, &color)) in word.chars.iter().zip(pattern.colors.iter()).enumerate() {
            let idx = chtoi(ch);
            match color {
                Colors::GRAY => data.set_gray(idx),
                Colors::YELLOW => data.set_yellow(idx, i),
                Colors::GREEN => data.set_green(idx, i),
                _ => unreachable!()
            }
        }

        return data;
    }

    // Determines if a given word matches the current pattern info
    pub fn matches(&self, word: &Word) -> bool {
        let mut counters = [0; MAT_WIDTH];

        // Stop immediately if one of the letters in the proposed
        // word cannot be in its current position
        for (i, &ch) in word.chars.iter().enumerate() {
            let idx = chtoi(ch);
            if self.matrix[idx][i] == MatrixData::NO {
                return false;
            }
            
            // Otherwise, update the counter
            counters[idx] += 1;
        }

        // When this point is reached, in theory, all letters in the
        // proposed word are allowed to be in their positions. However, we
        // must also check that the proposed word has at least as many of
        // the letters that we marked in yellow
        self.yellow_chars.iter().copied()
            .all(|idx| counters[idx] >= self.counters[idx])
    }

    // Auxiliary methods to update the internal data during initialization
    // Sets the entire row for this character to NO
    fn set_gray(&mut self, idx: usize) {
        (0..5).for_each(|i| self.matrix[idx][i] = MatrixData::NO);
    }

    // Sets only the character's position to NO, and increments
    // the counters for yellow characters
    fn set_yellow(&mut self, idx: usize, i: usize) {
        self.matrix[idx][i] = MatrixData::NO;
        self.counters[idx] += 1;

        if !self.yellow_chars.contains(&idx) {
            self.yellow_chars.push(idx);
        }
    }

    // Sets the entire column for this position to NO, except
    // for the character that was the green match
    fn set_green(&mut self, idx: usize, i: usize) {
        self.counters[idx] += 1;
        (0..MAT_WIDTH).for_each(|other_idx| self.matrix[other_idx][i] = MatrixData::NO);
        self.matrix[idx][i] = MatrixData::EXACT;
    }
}

pub fn chtoi(ch: char) -> usize {
    ch as usize - 'a' as usize
}