/* By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
  What is the 10 001st prime number? */

extern crate primal;

fn main() {
    let sieve = primal::Sieve::new(1_000_000_000);
    println!("The 10_001st prime is {}", sieve.nth_prime(10_001));
}
