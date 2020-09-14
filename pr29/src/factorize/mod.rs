extern crate primal;

pub fn factorize(number_to_factorize: &usize) -> Vec<usize> {
    let mut final_factors: Vec<usize> = Vec::new();
    let mut factorizing : usize = *number_to_factorize;
    if primal::is_prime(factorizing as u64) {
        final_factors.push(factorizing);
        return final_factors;
    }
    let (_, hi) = primal::estimate_prime_pi(factorizing as u64);
    let prime_list : Vec<usize> = primal::Primes::all().take(hi as usize).collect::<Vec<_>>();
    for prime_number in prime_list {
        'prime_attempts: loop {
            if factorizing % prime_number == 0 {
                final_factors.push(prime_number);
                factorizing /= prime_number;
            } else if factorizing == 1 {
                final_factors.sort();
                return final_factors;
            } else {
                break 'prime_attempts;
            }
        }
    }
    final_factors.clear();
    final_factors
}
