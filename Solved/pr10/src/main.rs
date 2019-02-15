extern crate primal;

fn main() {
    let sum = primal::Primes::all()
        .take_while(|p| *p < 2_000_000)
        .fold(0, |acc, x| acc + x);
    println!("The sum of all primes below 1 million is {}", sum);
}
