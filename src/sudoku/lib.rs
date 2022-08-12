#[derive(Debug, Clone, Copy)]
pub struct Cell(u32);

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
}

#[derive(Debug)]
pub struct Sudoku {
    state: [Cell; 81],
}

impl Sudoku {
    pub fn new(cells: [Cell; 81]) -> Self {
        Self { state: cells }
    }

    pub fn is_solved(&self) -> bool {
        self.state.iter().all(Cell::is_solved)
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
}
