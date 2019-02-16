// Solve the Sudoku puzzles found in po90_sudoku.txt and, as proof of solution, return
//  the sum of the first three cells (as a three digit number) across all 50 solutions
mod matrix;
mod matrix_load;

use matrix::matrices::Matrix;
use matrix_load::matrix_loader;

fn main() {
    let mut puzzles : Vec<Matrix> = matrix_loader::load_in_matrices().unwrap();
    for mat in puzzles {
        print!("{}", mat);
    }
}
