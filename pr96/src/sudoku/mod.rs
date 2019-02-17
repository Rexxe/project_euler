// #[path = "sudoku"]
mod pvt_setup;

use crate::matrix::Matrix;
use crate::sudoku::pvt_setup::*;

pub fn solve_sudoku(mut puzzle : &mut Matrix) -> Vec<Step> {

    let mut possible_vals : Vec<Vec<Option<Vec<u8>>>> =
        build_pvt(&puzzle);
    setup_pvt(&puzzle, &mut possible_vals);
    // println!("{:?}", possible_vals);
    let mut steps_taken : Vec<Step> = Vec::new();






    steps_taken
}



pub enum MethodType {
    SqrDed,
    RowDed,
    ColDed,
    Guess(u8),
}

pub struct Step<'a> {
    row: u8,
    col: u8,
    val: u8,
    method: MethodType,
    notes: &'a str,
}
