extern crate primal;

use primal::Sieve;

fn attempt_factorization(
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

pub fn factorize(&number_to_factorize: &u32, 
	&max_prime: &usize,
	initial_prime_count : &i32,
	prime_step_size : &i32
	) -> Vec<usize> {    
    let mut prime_count = *initial_prime_count;
    let mut final_factors: Vec<usize> = Vec::new();
    loop {
        if attempt_factorization(
            *&number_to_factorize as usize,
            *&max_prime,
            *&prime_count,
            &mut final_factors,
        ) == true
        {
            break;
        }
        prime_count += *prime_step_size;
    }
    final_factors
}

/*fn verify_divisibility(
    &final_result: &bigint::U512,
    &initial_factor: &u32,
    &final_factor: &u32,
) -> bool {    
    (initial_factor..=final_factor)
        .map(|test_factor| bigint::U512::from(test_factor))
        .all(|test_factor| final_result % test_factor == bigint::U512::from(0))
}*/