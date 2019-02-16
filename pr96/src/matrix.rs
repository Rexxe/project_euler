pub mod matrices{
    extern crate unicode_segmentation;

    use std::fmt;
    use unicode_segmentation::UnicodeSegmentation;

    #[derive(Clone)]
    pub struct Matrix {
        pub vals : [[u8; 9]; 9],
        pub name : String,
    }

    // fn for loading in data row by row
    impl Matrix {
        pub fn load_row(&mut self, row_to_load: &str, &row_index: &usize) -> Result<bool, String> {
            let digit_vector: Vec<&str> =
                UnicodeSegmentation::graphemes(row_to_load, true).collect::<Vec<&str>>();
            if digit_vector.len() != 9 {
                return Err(
                    format!("row_to_load is of length {} and must be of length 9",
                        row_to_load.len()));
            }
            let mut parser : String = String::with_capacity(1);
            let mut col_index : usize = 0;
            for digit in digit_vector {
                parser.push_str(digit);
                self.vals[row_index][col_index] = parser.parse::<u8>().unwrap();
                parser.clear();
                col_index+=1;
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
                write!(f, "{}", &format!("|{}{}{}|{}{}{}|{}{}{}|\n",
                    self.vals[i][0], self.vals[i][1], self.vals[i][2],
                    self.vals[i][3], self.vals[i][4], self.vals[i][5],
                    self.vals[i][6], self.vals[i][7], self.vals[i][8]))
                    .expect(&format!("Write of row {} failed", i));
            }
            Ok(())
        }
    }
}
