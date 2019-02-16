// Assumes file format of
/*
    Grid 01
    003020600
    900305001
    ...
    Grid 02
    ...
*/

pub mod matrix_loader {
    use crate::matrix::matrices::Matrix;
    use std::io::{BufReader,BufRead};
    use std::fs::File;
    use std::result;

    pub fn load_in_matrices() -> result::Result<Vec<Matrix>, String> {
        let mut vec : Vec<Matrix> = Vec::new();
        let file = File::open("p096_sudoku.txt").unwrap();
        let mut row_index : usize = 9;
        let mut current_matrix = Matrix { vals: [[0u8; 9]; 9], name: String::from("") };
        let mut first_pass : bool = true;
        for line_read in BufReader::new(file).lines() {
            match line_read {
                Ok(line) => {
                    if row_index == 9 {
                        if first_pass {
                            first_pass = false;
                            current_matrix.name = line;
                        }
                        else {
                            vec.push(current_matrix.clone());
                            current_matrix = Matrix { vals: [[0u8; 9]; 9], name: line };
                        }
                        row_index = 0;
                    }
                    else {
                        current_matrix.load_row(&line, &row_index).unwrap();
                        row_index+=1;
                    }
                }
                Err(e) => return Err(e.to_string())
            };
        }
        Ok(vec)
    }


}
