use crate::{Cell, Sudoku};

/// Given 9 cells, eliminate options in all given cells for each known cell.
pub fn uniqueness_eliminate(puz: &mut Sudoku, group: [usize; 9]) {
    let knowns = group
        .iter()
        .copied()
        .filter(|i| puz.state[*i].is_solved())
        .fold(0, |acc, i| acc | puz.state[i].0);

    let mask = !knowns;

    for i in group {
        if !puz.state[i].is_solved() {
            puz.state[i] = Cell(puz.state[i].0 & mask);
        }
    }
}

/// For each group, count the number of each possibility. If there is only one
/// of any given number, then that position must be that number.
pub fn single_possibility(puz: &mut Sudoku, group: [usize; 9]) {
    #[derive(Clone, Copy)]
    struct Poss {
        count: usize,
        index: usize,
    }
    let mut possibilities = [Poss { count: 0, index: 0 }; 9];

    for i in group {
        let cell = puz.state[i];
        for poss in 1..=9 {
            if (cell.0 & (1 << poss)) > 0 {
                possibilities[poss - 1].count += 1;
                possibilities[poss - 1].index = i;
            }
        }
    }

    for (val, p) in possibilities.iter().enumerate() {
        if p.count == 1 {
            puz.state[p.index] = Cell::known(val + 1);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{from_debug_grid, to_debug_grid};

    use super::*;
    use std::array;

    /// Cell with the given numbers eliminated.
    fn eliminated(ns: &[usize]) -> Cell {
        let inverted = ns
            .iter()
            .map(|c| Cell::known(*c).0)
            .reduce(|acc, c| acc | c)
            .unwrap_or(0);

        Cell(!inverted & Cell::unknown().0)
    }

    #[test]
    fn all_known() {
        let mut puz = Sudoku::new_empty();
        puz.set_row(0, array::from_fn(|i| Cell::known(i + 1)));
        puz.apply_to_groups(uniqueness_eliminate);
        let expected = from_debug_grid(r"
            1-------- -2------- --3------ ---4----- ----5---- -----6--- ------7-- -------8- --------9 
            ---456789 ---456789 ---456789 123---789 123---789 123---789 123456--- 123456--- 123456--- 
            ---456789 ---456789 ---456789 123---789 123---789 123---789 123456--- 123456--- 123456--- 
            -23456789 1-3456789 12-456789 123-56789 1234-6789 12345-789 123456-89 1234567-9 12345678- 
            -23456789 1-3456789 12-456789 123-56789 1234-6789 12345-789 123456-89 1234567-9 12345678- 
            -23456789 1-3456789 12-456789 123-56789 1234-6789 12345-789 123456-89 1234567-9 12345678- 
            -23456789 1-3456789 12-456789 123-56789 1234-6789 12345-789 123456-89 1234567-9 12345678- 
            -23456789 1-3456789 12-456789 123-56789 1234-6789 12345-789 123456-89 1234567-9 12345678- 
            -23456789 1-3456789 12-456789 123-56789 1234-6789 12345-789 123456-89 1234567-9 12345678-"
        ).unwrap();

        assert_eq!(expected, puz);
    }

    #[test]
    fn all_unknown() {
        let mut puz = Sudoku::new_empty();
        puz.set_row(0, [Cell::unknown(); 9]);
        println!("{}", to_debug_grid(&puz));
        puz.apply_to_groups(uniqueness_eliminate);
        let expected = from_debug_grid(r"
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 
            123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789",
        )
        .unwrap();

        assert_eq!(expected, puz);
    }

    // #[test]
    // fn one_known() {
    //     let mut before = [Cell::unknown(); 9];
    //     before[3] = Cell::known(1);

    //     let after = uniqueness_eliminate(before);
    //     let mut expected = [eliminated(&[1]); 9];
    //     expected[3] = Cell::known(1);

    //     assert_eq!(after, expected);
    // }

    // #[test]
    // fn two_known() {
    //     let mut before = [Cell::unknown(); 9];
    //     before[3] = Cell::known(1);
    //     before[4] = Cell::known(2);

    //     let after = uniqueness_eliminate(before);
    //     let mut expected = [eliminated(&[1, 2]); 9];
    //     expected[3] = Cell::known(1);
    //     expected[4] = Cell::known(2);

    //     assert_eq!(after, expected);
    // }
}
