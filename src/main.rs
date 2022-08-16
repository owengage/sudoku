use std::fs::File;
use std::io::{self, BufRead};

use sudoku::{
    from_digit_line, single_possibility, to_debug_grid, to_pretty_grid, uniqueness_eliminate,
};

fn main() -> anyhow::Result<()> {
    let file = io::BufReader::new(File::open("sudoku.csv").unwrap());
    let mut solved = 0;

    for line in file.lines().skip(1).flatten() {
        let mut fields = line.split(',');
        let mut puzzle = from_digit_line(fields.next().unwrap());
        let solution = from_digit_line(fields.next().unwrap());

        if !solution.is_solved() {
            println!("Should be solved: {solution:?}");
            break;
        }

        if puzzle.is_solved() {
            println!("Should NOT be solved: {puzzle:?}");
            break;
        }

        let mut score = usize::MAX;
        loop {
            let new_score = puzzle.score();
            if puzzle.is_solved() {
                solved += 1;
                println!("Solved {solved}");
                assert_eq!(puzzle, solution);
                break;
            }

            if score == new_score {
                println!("FAILED TO SOLVE:\n{}", to_pretty_grid(&puzzle));
                println!("{}", to_debug_grid(&puzzle));

                let mut placeholder = String::new();
                std::io::stdin().read_line(&mut placeholder)?;

                break;
            }

            score = new_score;
            puzzle.apply_to_groups(uniqueness_eliminate);
            puzzle.apply_to_groups(single_possibility);
        }
    }

    Ok(())
}
