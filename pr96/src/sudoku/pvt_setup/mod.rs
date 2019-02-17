use crate::matrix::Matrix;

pub fn setup_pvt(puzzle : &Matrix, mut pvt : &Vec<Vec<Option<Vec<u8>>>>) {
    // Now we actually determine the possible values for each position





}

pub fn pvt_cell_calc_pvs(puzzle : &Matrix, mut pvt : &Vec<Vec<Option<Vec<u8>>>>, row: usize, col: usize) {
    if puzzle.vals[row][col] != 0 { return; }
    // All values are considered possible and then removed under the following conditions
    let mut possible_vals = (0..=9).collect::<Vec<u8>>();
    // Vec::with_capacity(9)

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
