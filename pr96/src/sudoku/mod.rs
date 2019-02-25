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
    // // For temp testing purposes
    // let test_step = Step {
    //     row: 0,
    //     col: 0,
    //     val: 4,
    //     method: MethodType::Guess(0),
    //     notes: None,
    //     taken: true,
    // };
    //
    // let test_step_2 = Step {
    //     row: 1,
    //     col: 1,
    //     val: 5,
    //     method: MethodType::Ded,
    //     notes: None,
    //     taken: true,
    // };
    //
    // puzzle.take_action(&mut vec![test_step, test_step_2], &mut all_steps_taken);
    //
    // puzzle.undo_actions_to_concrete_state(&mut all_steps_taken, &1);
    //
    // println!("{:#?}",all_steps_taken);
    // temp
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
    pub fn is_guess_later(&self, &guess_pos : &i32) -> bool {
        match self {
            MethodType::Guess(ind) => {
                if ind >= &(guess_pos as u8) {
                    true
                }
                else {
                    false
                }
            },
            _ => false,
        }
    }
}
