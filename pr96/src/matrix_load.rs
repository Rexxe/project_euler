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

    pub fn load_in_matrices() -> Vec<Matrix> {
        let mut vec : Vec<Matrix> = Vec::new();
        vec.push(Matrix { vals: [[0u8; 9]; 9], name: String::from("Hello world") });
        // let file = File::open("p096_sudoku.txt").unwrap();
        // for line in BufReader::new(file).lines() {
        //     println!("{}", line.unwrap());
        //
        //
        //
        //
        // }






        vec
    }


}
