use std::fs::File;
use std::io::{self, BufRead};

use anyhow::Result;
use sudoku::{Cell, Sudoku};

fn parse_sudoku_line(line: &str) -> Result<Sudoku> {
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
    Ok(Sudoku::new(s))
}

struct Pair {
    puzzle: Sudoku,
    solution: Sudoku,
}

fn main() {
    let file = io::BufReader::new(File::open("sudoku.csv").unwrap());
    let mut puzzles = vec![];

    for line in file.lines().skip(1).flatten() {
        let mut fields = line.split(',');
        let puzzle = parse_sudoku_line(fields.next().unwrap()).unwrap();
        let solution = parse_sudoku_line(fields.next().unwrap()).unwrap();

        if !solution.is_solved() {
            println!("{solution:?}");
            break;
        }

        puzzles.push(Pair { puzzle, solution });
    }

    let count = puzzles.len();
    println!("processed {count}");
}
