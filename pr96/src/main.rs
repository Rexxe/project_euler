// Solve the Sudoku puzzles found in po90_sudoku.txt and, as proof of solution, return
//  the sum of the first three cells (as a three digit number) across all 50 solutions
// #[path = "matrices/matrix.rs"]
mod matrix;
#[path = "matrix/matrix_load.rs"]
mod matrix_load;
mod sudoku;

use matrix::Matrix;
use matrix_load::load_in_matrices;
use sudoku::solve_sudoku;

fn main() {
    let mut matrices: Vec<Matrix> = load_in_matrices().unwrap();
    let mut matrix_iter = matrices.iter_mut();
    while let Some(mut mat) = matrix_iter.next() {
        // println!("{}", *mat);
        solve_sudoku(&mut mat);
        println!("{}", *mat);
        
        break; // For the moment, break on the first loop so we can debug more easily
    }
}
