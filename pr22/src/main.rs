extern crate memmap;

use std::env;
use std::fs::{ File, Metadata};
use std::fs;
use std::path::PathBuf;

use memmap::Mmap;

fn main() {
    let curr_path = match env::current_dir() {
        Ok(path) => path,
        Error => {
            return;
        }
    };


    // env::current_dir()?;

    let file : File = File::open("./p022_names.txt").unwrap();
    let file_metadata : Metadata = fs::metadata("./p022_names.txt").unwrap();
    let map = unsafe { Mmap::map(&file).unwrap() };
    // Write the positions of all commas in the file
    for i  in 0 as usize..file_metadata.len() as usize {
        if &map[i] == &(b","[0]) {
            println!("Comma found at byte {}", i);
        }
    }





}
