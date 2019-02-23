use crate::matrix::Matrix;

pub fn setup_pvt(puzzle : &Matrix, mut pvt : &mut Vec<Vec<Option<Vec<u8>>>>) {
    // Now we actually determine the possible values for each position
    pvt_full_calc_pvs(&puzzle, &mut pvt);
}

pub fn pvt_full_calc_pvs(puzzle : &Matrix, mut pvt : &mut Vec<Vec<Option<Vec<u8>>>>) {
    for r in (0 as usize)..(9 as usize) {
        for c in (0 as usize)..(9 as usize) {
            pvt_cell_calc_pvs(&puzzle, &mut pvt, &r, &c);
        }
    }
}

pub fn pvt_cell_calc_pvs(puzzle : &Matrix, pvt : &mut Vec<Vec<Option<Vec<u8>>>>, &row: &usize, &col: &usize) {
    if puzzle.vals[row][col] != 0 { return; }
    // All values are considered possible and then removed under the following conditions
    let mut possible_vals : &mut Vec<u8> = match &mut pvt[row][col] {
        Some(previous_vals) => {
            previous_vals.clear();
            previous_vals.append(&mut (1..=9).collect::<Vec<u8>>());
            previous_vals
        },
        None => {
            return;
        },
    };

    pvt_cell_check_col(&puzzle, &mut possible_vals, &row, &col);
    pvt_cell_check_row(&puzzle, &mut possible_vals, &row, &col);
    pvt_cell_check_sqr(&puzzle, &mut possible_vals, &row, &col);

    if possible_vals.len() == 0 { pvt[row][col] = None; }
}

pub fn pvt_cell_check_sqr(puzzle : &Matrix,
    possible_vals : &mut Vec<u8>,
    &row: &usize, &col: &usize) {

    let row_start : usize = row - row % 3;
    let col_start : usize = col - col % 3;

    for r in row_start..(row_start+3) {
        for c in col_start..(col_start+3) {
            if r == row && c == col { continue; }
            possible_vals.retain(|&x| x != puzzle.vals[r][c]);
        }
    }
}

pub fn pvt_cell_check_col(puzzle : &Matrix,
    possible_vals : &mut Vec<u8>,
    &row: &usize, &col: &usize) {

    for r in 0..9 {
        if r == row { continue; }
        possible_vals.retain(|&x| x != puzzle.vals[r][col]);
    }
}

pub fn pvt_cell_check_row(puzzle : &Matrix,
    possible_vals : &mut Vec<u8>,
    &row: &usize, &col: &usize) {

    for c in 0..9 {
        if c == col { continue; }
        possible_vals.retain(|&x| x != puzzle.vals[row][c]);
    }
}

pub fn build_pvt(puzzle : &Matrix) -> Vec<Vec<Option<Vec<u8>>>> { // PVT = Possible Values Table
    // Row<Col<Some(PossibleValues)/None
    // Vec<Vec<Option<Vec<u8>>>>
    let mut possible_vals : Vec<Vec<Option<Vec<u8>>>> = Vec::with_capacity(9);
    let mut row_vector : Vec<Option<Vec<u8>>> = Vec::with_capacity(9);
    let mut cell : Option<Vec<u8>>;
    for r in 0..9 {
        for c in 0..9 {
            if puzzle.vals[r][c] == 0 {
                cell = Some(Vec::with_capacity(9))
            }
            else {
                cell = None;
            }
            row_vector.push(cell.clone());
        }
        possible_vals.push(row_vector.clone());
        row_vector.clear();
    }
    possible_vals
}
