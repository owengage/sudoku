struct Cell(u32);

impl Cell {
    fn solved(n: usize) -> Self {
        debug_assert!((1..=9).contains(&n));
        Self(1 << n)
    }

    fn unknown() -> Self {
        #[allow(clippy::unusual_byte_groupings)]
        Self(0b111_111_111_0)
    }
}

struct Sudoku {
    state: [Cell; 81],
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_rep() {
        assert_eq!(Cell::solved(1).0, 0b10);
        assert_eq!(Cell::solved(3).0, 0b1000);
        assert_eq!(Cell::unknown().0, 0b1111111110)
    }
}
