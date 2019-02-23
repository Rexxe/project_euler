use crate::sudoku::{Step, MethodType};

pub fn trivial_ded<'a>(possible_vals : &mut Vec<Vec<Option<Vec<u8>>>>
    ) -> Option<Vec<Step<'a>>> {

    let mut steps_taken : Vec<Step> = Vec::new();
    for r in (0 as usize)..(9 as usize) {
        for c in (0 as usize)..(9 as usize) {
            match &possible_vals[r][c] {
                Some(cell_pvs) => {
                    if cell_pvs.len() == 1 {
                        let step = Step {
                            row : r,
                            col : c,
                            val : cell_pvs[0],
                            method : MethodType::Ded,
                            notes : None,
                            taken : false,
                        };
                        steps_taken.push(step);
                    }
                },
                None => (),
            };
        }
    }

    if steps_taken.len() != 0 { Some(steps_taken) }
    else { None }
}
