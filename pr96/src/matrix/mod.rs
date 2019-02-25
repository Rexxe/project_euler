extern crate unicode_segmentation;

use crate::sudoku::{Step, MethodType};
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
pub struct Matrix {
    pub vals: [[u8; 9]; 9],
    pub name: String,
}

impl Matrix {
    // fn for taking an action
    pub fn take_action<'a>(
        &mut self,
        steps_to_take: &mut Vec<Step<'a>>,
        steps_taken: &mut Vec<Step<'a>>,
    ) {
        for step in steps_to_take.clone() {
            self.vals[step.row][step.col] = step.val;
        }
        steps_taken.append(steps_to_take);
    }

    pub fn undo_action(&mut self, step: &Step) {
        self.vals[step.row][step.col] = 0;
    }

    pub fn undo_actions_to_concrete_state<'a>(&mut self,
        all_steps_taken: &mut Vec<Step<'a>>,
        guess_pos: &i32) {

        let concrete_pos : usize = match all_steps_taken.iter().enumerate()
            .skip_while(|(pos, s)| !s.method.is_guess_later(guess_pos)).take(1).next() {

            Some(pos) => pos.0,
            None => return,
        };
        for step_pos in concrete_pos..all_steps_taken.len() {
            self.undo_action(&all_steps_taken[step_pos]);
        }
        all_steps_taken.drain(concrete_pos..);
    }

    // fn for loading in data row by row
    pub fn load_row(&mut self, row_to_load: &str, &row_index: &usize) -> Result<bool, String> {
        let digit_vector: Vec<&str> =
            UnicodeSegmentation::graphemes(row_to_load, true).collect::<Vec<&str>>();
        if digit_vector.len() != 9 {
            return Err(format!(
                "row_to_load is of length {} and must be of length 9",
                row_to_load.len()
            ));
        }
        let mut parser: String = String::with_capacity(1);
        let mut col_index: usize = 0;
        for digit in digit_vector {
            parser.push_str(digit);
            self.vals[row_index][col_index] = parser.parse::<u8>().unwrap();
            parser.clear();
            col_index += 1;
        }
        Ok(true)
    }
}

// fn for Display Print to console
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &format!("'{}'\n", self.name))
            .expect(&format!("Failed to write matrix name '{}", self.name));
        'row_write: for i in 0..=9 {
            if i % 3 == 0 {
                write!(f, "{}", "-------------\n")
                    .expect(&format!("Write of pre-row dashes {} failed", i));
                if i == 9 {
                    continue 'row_write;
                }
            }
            write!(
                f,
                "{}",
                &format!(
                    "|{}{}{}|{}{}{}|{}{}{}|\n",
                    match self.vals[i][0] {
                        0 => String::from(" "),
                        _ => self.vals[i][0].to_string(),
                    },
                    match self.vals[i][1] {
                        0 => String::from(" "),
                        _ => self.vals[i][1].to_string(),
                    },
                    match self.vals[i][2] {
                        0 => String::from(" "),
                        _ => self.vals[i][2].to_string(),
                    },
                    match self.vals[i][3] {
                        0 => String::from(" "),
                        _ => self.vals[i][3].to_string(),
                    },
                    match self.vals[i][4] {
                        0 => String::from(" "),
                        _ => self.vals[i][4].to_string(),
                    },
                    match self.vals[i][5] {
                        0 => String::from(" "),
                        _ => self.vals[i][5].to_string(),
                    },
                    match self.vals[i][6] {
                        0 => String::from(" "),
                        _ => self.vals[i][6].to_string(),
                    },
                    match self.vals[i][7] {
                        0 => String::from(" "),
                        _ => self.vals[i][7].to_string(),
                    },
                    match self.vals[i][8] {
                        0 => String::from(" "),
                        _ => self.vals[i][8].to_string(),
                    },
                )
            )
            .expect(&format!("Write of row {} failed", i));
        }
        Ok(())
    }
}
