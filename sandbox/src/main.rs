extern crate fine_grained;
extern crate primal;

use fine_grained::Stopwatch;

fn main() {
    let mut stopwatch = Stopwatch::start_new();
    let mut p = 0;
    for k in 1.. {
        p = k * (1 << (27 + 1)) + 1;
        if primal::is_prime(p) { break;}
    }
    println!("The prime in question is {}.  Finding it took {}", p, stopwatch);
    stopwatch.stop();
}
