use anyhow::Context;

use crate::{Cell, Sudoku};

/// Callers responsibility to make sure it's a 81 long str of digits.
pub fn from_digit_line(line: &str) -> Sudoku {
    let cells = line.chars().map(|c| c.to_digit(10).unwrap()).map(|d| {
        if d == 0 {
            Cell::unknown()
        } else {
            Cell::known(d as usize)
        }
    });

    let mut i = 0;
    let mut s = [Cell::unknown(); 81];
    for cell in cells {
        s[i] = cell;
        i += 1;
    }

    debug_assert!(i == 81);
    Sudoku::new(s)
}

pub fn to_debug_grid(puz: &Sudoku) -> String {
    let mut s = String::new();
    for (i, cell) in puz.state.iter().enumerate() {
        if i % 9 == 0 {
            s += "\n";
        }
        if i % 27 == 0 {
            s += "\n";
        }
        // print the cell
        for digit in 1..=9 {
            if cell.can_be(digit) {
                s += &digit.to_string();
            } else {
                s += "-"
            }
        }
        s += " ";
    }

    s
}

pub fn from_debug_grid(grid: &str) -> anyhow::Result<Sudoku> {
    let cs: anyhow::Result<Vec<_>> = grid
        .split('\n')
        .filter(|line| !line.is_empty())
        .take(9)
        .flat_map(|line| line.trim().split(' '))
        .map(|digits| -> anyhow::Result<Cell> {
            let mut cell = 0usize;
            for digit in digits.chars() {
                if digit == '-' {
                    continue;
                }
                let digit = digit
                    .to_digit(10)
                    .context("debug grid contained none-digit")?;
                cell |= 1 << digit;
            }

            Ok(Cell(cell))
        })
        .collect();

    Ok(Sudoku::new(cs?.as_slice().try_into()?))
}

pub fn to_pretty_grid(puz: &Sudoku) -> String {
    let mut s = String::new();
    for (i, cell) in puz.state.iter().enumerate() {
        if i % 3 == 0 {
            s += " ";
        }
        if i % 9 == 0 {
            s += "\n";
        }
        if i % 27 == 0 {
            s += "\n";
        }
        // print the cell
        match cell.value() {
            Some(v) => s += &v.to_string(),
            None => s += ".",
        }
        s += " ";
    }

    s
}
