extern crate primal;

use primal::Sieve;

pub fn attempt_factorization(
    mut number_to_factorize: usize,
    max_prime: usize,
    prime_count: i32,
    final_factors: &mut Vec<usize>,
) -> bool {
    let sieve = Sieve::new(max_prime);
    let prime_list = (1..=prime_count)
        .map(|x| sieve.nth_prime(x as usize))
        .collect::<Vec<_>>();
    for prime_number in prime_list {
        'prime_attempts: loop {
            // println!("Test prime {:?}", prime_number);
            if number_to_factorize % prime_number == 0 {
                // println!("Prime factor found {:?}", prime_number);
                final_factors.push(prime_number);
                number_to_factorize /= prime_number;
            // println!("Full reduced to {:?}", number_to_factorize);
            } else if number_to_factorize == 1 {
                return true;
            } else {
                break 'prime_attempts;
            }
        }
    }
    final_factors.clear();
    false
}
