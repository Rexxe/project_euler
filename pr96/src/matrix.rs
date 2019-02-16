pub mod matrices{
    use std::fmt;

    pub struct Matrix {
        pub vals : [[u8; 9]; 9],
        pub name : String,
    }

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", &format!("'{}'\n", self.name));
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
