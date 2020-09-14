extern crate primal;

use primal::Sieve;

pub fn factorize(&number_to_factorize: &usize, sieve: &Sieve) -> Vec<usize> {
    let mut prime_count = 20;
    let mut final_factors: Vec<usize> = Vec::new();
    loop {
        if attempt_factorization(
            *&number_to_factorize,
            &sieve,
            *&prime_count,
            &mut final_factors,
        ) == true
        {
            break;
        }
        prime_count += 100;
    }
    final_factors
}

pub fn attempt_factorization(
    mut number_to_factorize: usize,
    sieve: &Sieve,
    prime_count: i32,
    final_factors: &mut Vec<usize>,
) -> bool {
    let prime_list = (1..=prime_count)
        .map(|x| sieve.nth_prime(x as usize))
        .collect::<Vec<_>>();
    for prime_number in prime_list {
        'prime_attempts: loop {
            if number_to_factorize % prime_number == 0 {
                final_factors.push(prime_number);
                number_to_factorize /= prime_number;
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
