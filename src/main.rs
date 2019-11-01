mod repr;
mod solve;

use solve::*;

fn main() {
    if let Some(sudoku) = std::env::args().nth(1) {
        if let Some(solution) = solve_sudoku(sudoku.as_str()) {
            println!("{}", solution);
        } else {
            println!("Cannot solve sudoku:\n\n{}", sudoku);
        }
    } else {
        println!("No input");
    }
}
