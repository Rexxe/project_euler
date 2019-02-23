mod pvt_setup;
#[path = "pvt_solving/solver_ded.rs"]
mod solver_ded;

use crate::matrix::Matrix;
use crate::sudoku::pvt_setup::*;
use solver_ded::*;

pub fn solve_sudoku(puzzle : &mut Matrix) -> Vec<Step> {
    let mut possible_vals : Vec<Vec<Option<Vec<u8>>>> = build_pvt(&puzzle);

    let mut all_steps_taken : Vec<Step> = Vec::new();
    'Solver_loop: loop {
        setup_pvt(&puzzle, &mut possible_vals);
        match trivial_ded(&mut possible_vals) {
            Some(mut steps_to_take) => {
                puzzle.take_action(&mut steps_to_take, &mut all_steps_taken);
                continue 'Solver_loop;
            },
            None => break 'Solver_loop,
        };

    }

    all_steps_taken
}

#[derive(Clone,Debug)]
pub struct Step<'a> {
    pub row: usize,
    pub col: usize,
    pub val: u8,
    pub method: MethodType,
    pub notes: Option<&'a str>,
    pub taken: bool,
}

#[derive(Clone,Debug)]
pub enum MethodType {
    Ded,
    Guess(u8),
    FinalGuess(u8),
}
