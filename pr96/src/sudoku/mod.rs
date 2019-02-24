mod pvt_setup;
// #[path = "pvt_solving/solver_ded.rs"]
mod pvt_solving;

use crate::matrix::Matrix;
use crate::sudoku::pvt_setup::*;
use pvt_solving::*;

pub fn solve_sudoku(puzzle: &mut Matrix) -> Vec<Step> {
    let mut possible_vals: Vec<Vec<Option<Vec<u8>>>> = build_pvt(&puzzle);

    let mut all_steps_taken: Vec<Step> = Vec::new();
    // Steps which were guessed but ended up causing a contradiction
    let mut forbidden_steps: Vec<Step> = Vec::new();

    'Solver_loop: loop {
        // (Re)determine possible values
        setup_pvt(&puzzle, &mut possible_vals);




        // Perform deterministic fills
        match deduct(&mut possible_vals) {
            Some(mut steps_to_take) => {
                puzzle.take_action(&mut steps_to_take, &mut all_steps_taken);
                continue 'Solver_loop;
            }
            None => break 'Solver_loop,
        };
        // Begin guess work




    }

    all_steps_taken
}

#[derive(Clone, Debug)]
pub struct Step<'a> {
    pub row: usize,
    pub col: usize,
    pub val: u8,
    pub method: MethodType,
    pub notes: Option<&'a str>,
    pub taken: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MethodType {
    Ded,
    Guess(u8),
    GuessConcreted(u8),
}

impl MethodType {
    pub fn IsGuess(&self) -> bool {
        match self {
            Guess(_) => true,
            _ => false,
        }
    }
}
