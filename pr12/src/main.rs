/* The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle
number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
Let us list the factors of the first seven triangle numbers:
     1: 1
     3: 1,3
     6: 1,2,3,6
    10: 1,2,5,10
    15: 1,3,5,15
    21: 1,3,7,21
    28: 1,2,4,7,14,28
We can see that 28 is the first triangle number to have over five divisors.
What is the value of the first triangle number to have over five hundred divisors? */

extern crate primal;

mod factorize;

fn main() {
    let mut counter = TriangleCounter::new();
    let mut current_number: usize;
    let mut current_max_factor_count: usize;
    current_max_factor_count = 0;

    loop {
        current_number = counter.next().unwrap();
        let final_factors: Vec<usize> = counter.get_prime_factors();

        if current_max_factor_count < final_factors.len() {
            current_max_factor_count = final_factors.len();
            println!(
                "Number '{}' introduces New Max Factor Count: {} '{:?}'",
                current_number, current_max_factor_count, final_factors
            );
        }

        if final_factors.len() > 500 {
            // We are finished
            println!("Found desired number: {}", current_number);
            break;
        }

        // if max_so_far > current_number {
        //     println!("Wrapping began at {:?}", max_so_far);
        // }

        // if counter.increment % 1_000_000_000 == 0 {
        //     println!("{:?}", counter.increment);
        // }
    }

    // println!("Hello, world!");
}

struct TriangleCounter {
    increment: usize,
    current: usize,
    sieve_prime: primal::Sieve,
    sieve_from: usize,
    prime_count: i32,
}

impl TriangleCounter {
    fn new() -> TriangleCounter {
        TriangleCounter {
            increment: 2,
            current: 1,
            sieve_prime: primal::Sieve::new(3),
            sieve_from: 1,
            prime_count: 1,
        }
    }
}

impl Iterator for TriangleCounter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current += self.increment;
        self.increment += 1;
        // Form/update the prime sieve which will be usable externally
        if self.current > self.sieve_from {
            println!("p0: {:?}", self.current * 100);
            self.prime_count = primal::estimate_prime_pi((self.current * 100) as u64).1 as i32;
            println!("p1: {:?}", self.prime_count);
            self.sieve_from = primal::estimate_nth_prime(self.prime_count as u64).1 as usize;
            println!("p2: {:?}", self.prime_count);
            self.sieve_prime = primal::Sieve::new(self.sieve_from);
            println!("p3: {:?}", self.prime_count);
            println!("Prime count extended {:?}", self.sieve_from);
        }

        Some(self.current)
    }
}

impl TriangleCounter {
    pub fn get_prime_factors(&mut self) -> Vec<usize> {
        let /* mut */ prime_factors: Vec<usize> = factorize::factorize(&self.current, &self.sieve_prime);
        //prime_factors.sort();
        prime_factors
    }
}
