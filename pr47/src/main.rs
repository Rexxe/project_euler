/* The first two consecutive numbers to have two distinct prime factors are:
      14 = 2 × 7
      15 = 3 × 5
  The first three consecutive numbers to have three distinct prime factors are:
    644 = 2² × 7 × 23
    645 = 3 × 5 × 43
    646 = 2 × 17 × 19.
  Find the first four consecutive integers to have four distinct prime factors each.
    What is the first of these numbers? */
extern crate primal;

mod factorize;

// use primal::{Sieve};

fn main() {

    let mut num_current : u32 = 210; // First natural with 4 distinct factors is 210
    let mut factors_obtained : Vec<u32>;
    let mut num_1 : u32 = 0;
    let mut num_2 : u32 = 0;
    let mut num_3 : u32 = 0;
    'main_loop : loop {
        factors_obtained = get_distinct_factors(&num_current);
        if factors_obtained.len() == 4 {
            // println!("{:<5} has four unique factors : {:?}", num_current, factors_obtained);
            if num_1 == num_current - 1 {
                num_2 = num_current;
            } else if num_1 == num_current - 2
                      && num_2 == num_current - 1 {
                num_3 = num_current;
            } else if num_1 == num_current - 3
                      && num_2 == num_current - 2
                      && num_3 == num_current - 1 {
                break 'main_loop;
            } else {
                num_1 = num_current;
            }
        }
        factors_obtained.clear();
        num_current += 1;
    }
    println!("The first set of four consecutive natural numbers with four distinct factors are \
        {}, {}, {}, and {}", num_1, num_2, num_3, num_current);
}

fn get_distinct_factors(num_in : &u32) -> Vec<u32> {
    let mut factors = factorize::factorize(num_in, &100_000, &1000, &1000)
        .iter()
        .map(|&e| e as u32)
        .collect::<Vec<u32>>();
    factors.sort();
    factors.dedup();
    factors
}
