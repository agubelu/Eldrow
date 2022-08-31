use crate::common::{Word, Pattern, Colors};

type MatrixCell = u8;

struct MatrixData;

impl MatrixData {
    pub const UNKNOWN: MatrixCell = 0;
    pub const MISS: MatrixCell = 1;
    pub const EXACT: MatrixCell = 2;
}

// Auxiliary data used to quickly determine whether a possible solution
// matches a color pattern produced by a given word
pub struct MatchInfo {
    pub matrix: Matrix<MatrixCell>,
    pub counters: Vec<u8>,
    pub yellow_chars: Vec<usize>
}

// A 2D-like structure backed by a linear vector
pub struct Matrix<T: Copy> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl MatchInfo {
    pub fn from_word_match(word: &Word, pattern: &Pattern, n_chars: usize) -> Self {
        let matrix = Matrix::new(n_chars, 5, MatrixData::UNKNOWN);
        let counters = vec![0; n_chars];
        let yellow_chars = Vec::with_capacity(5);

        // Initialization
        let mut data = Self { matrix, counters, yellow_chars };
        for (i, (&ch, &color)) in word.chars.iter().zip(pattern.colors.iter()).enumerate() {
            let idx = ch as usize;
            match color {
                Colors::GRAY => {
                    // Only set the entire row to gray if the
                    // character has not appeared previously in yellow
                    if !data.yellow_chars.contains(&idx) {
                        data.set_gray(idx)
                    } else {
                        // Otherwise, we can only be sure that the
                        // character does not appear in that
                        // particular location
                        data.matrix.set(idx, i, MatrixData::MISS);
                    }
                },
                Colors::YELLOW => data.set_yellow(idx, i),
                Colors::GREEN => data.set_green(idx, i),
                _ => unreachable!()
            }
        }

        data
    }

    // Determines if a given word matches the current pattern info
    pub fn matches(&self, word: &Word) -> bool {
        let mut counters = vec![0; self.counters.len()];

        // Stop immediately if one of the letters in the proposed
        // word cannot be in its current position
        for (i, &ch) in word.chars.iter().enumerate() {
            let idx = ch as usize;
            if self.matrix.get(idx, i) == MatrixData::MISS {
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
    // Sets the entire row for this character to NO, taking care not
    // to overwrite green/EXACT matches
    fn set_gray(&mut self, idx: usize) {
        (0..5).for_each(|i| {
            if self.matrix.get(idx, i) != MatrixData::EXACT {
                self.matrix.set(idx, i, MatrixData::MISS);
            }
        });
    }

    // Sets only the character's position to NO, and increments
    // the counters for yellow characters
    fn set_yellow(&mut self, idx: usize, i: usize) {
        self.matrix.set(idx, i, MatrixData::MISS);
        self.counters[idx] += 1;

        if !self.yellow_chars.contains(&idx) {
            self.yellow_chars.push(idx);
        }
    }

    // Sets the entire column for this position to NO, except
    // for the character that was the green match
    fn set_green(&mut self, idx: usize, i: usize) {
        self.counters[idx] += 1;
        (0..self.matrix.rows).for_each(|other_idx| self.matrix.set(other_idx, i, MatrixData::MISS));
        self.matrix.set(idx, i, MatrixData::EXACT);
    }
}

impl<T: Copy> Matrix<T> {
    pub fn new(rows: usize, cols: usize, initial: T) -> Self {
        Self { rows, cols, data: vec![initial; rows * cols] }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let i = self.index(row, col);
        self.data[i]
    }
    
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let i = self.index(row, col);
        self.data[i] = value;
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}