mod basic;
mod parse;

pub use basic::*;
pub use parse::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell(usize);

impl Cell {
    pub fn known(n: usize) -> Self {
        debug_assert!((1..=9).contains(&n));
        Self(1 << n)
    }

    pub fn unknown() -> Self {
        #[allow(clippy::unusual_byte_groupings)]
        Self(0b111_111_111_0)
    }

    pub fn is_solved(&self) -> bool {
        self.0.count_ones() == 1
    }

    fn can_be(&self, digit: usize) -> bool {
        (self.0 & (1 << digit)) > 0
    }

    fn value(&self) -> Option<usize> {
        if self.is_solved() {
            Some(self.0.trailing_zeros() as usize)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sudoku {
    pub(crate) state: [Cell; 81],
}

const fn row_indicies(i: usize) -> [usize; 9] {
    let start = i * 9;
    [
        start,
        start + 1,
        start + 2,
        start + 3,
        start + 4,
        start + 5,
        start + 6,
        start + 7,
        start + 8,
    ]
}

const fn col_indicies(i: usize) -> [usize; 9] {
    let stride = 9;
    let offset = i;
    [
        offset,
        offset + stride,
        offset + stride * 2,
        offset + stride * 3,
        offset + stride * 4,
        offset + stride * 5,
        offset + stride * 6,
        offset + stride * 7,
        offset + stride * 8,
    ]
}

const fn chunk_indicies(i: usize) -> [usize; 9] {
    let loff = (i % 3) * 3;
    let toff = (i / 3) * 3 * 9;

    [
        toff + loff,
        toff + loff + 1,
        toff + loff + 2,
        toff + 9 + loff,
        toff + 9 + loff + 1,
        toff + 9 + loff + 2,
        toff + 18 + loff,
        toff + 18 + loff + 1,
        toff + 18 + loff + 2,
    ]
}

impl Sudoku {
    pub fn new(cells: [Cell; 81]) -> Self {
        Self { state: cells }
    }

    pub fn new_empty() -> Self {
        Self {
            state: [Cell::unknown(); 81],
        }
    }

    pub fn is_solved(&self) -> bool {
        self.state.iter().all(Cell::is_solved)
        
    }

    /// Score is the total number of possible values left for each cell, minus
    /// the 81 that would be expected from a complete sudoku.
    pub fn score(&self) -> usize {
        self.state
            .iter()
            .map(|c| c.0.count_ones() as usize)
            .sum::<usize>()
            - 81
    }

    pub fn apply_to_groups<F>(&mut self, f: F)
    where
        F: Fn(&mut Sudoku, [usize; 9]),
    {
        for i in 0..9 {
            f(self, row_indicies(i));
        }

        for i in 0..9 {
            f(self, col_indicies(i))
        }

        for i in 0..9 {
            f(self, chunk_indicies(i))
        }

        // TODO: cols, chunks
    }

    pub(crate) fn set_row(&mut self, row: usize, cells: [Cell; 9]) {
        debug_assert!((0..9).contains(&row));
        let group = row_indicies(row);
        for i in group {
            self.state[i] = cells[i];
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Cell {
        self.state[row * 9 + col]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_rep() {
        assert_eq!(Cell::known(1).0, 0b10);
        assert_eq!(Cell::known(3).0, 0b1000);
        assert_eq!(Cell::unknown().0, 0b1111111110)
    }

    #[test]
    fn nop_apply() {
        let mut puz = Sudoku::new([Cell::unknown(); 81]);
        let copy = puz.clone();
        puz.apply_to_groups(|_, _| {});
        assert_eq!(copy, puz);
    }
}
