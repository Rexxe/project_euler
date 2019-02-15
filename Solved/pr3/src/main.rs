/* The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ? */

extern crate primal;

use primal::Sieve;

fn main() {
    let number_to_factorize: usize = 600_851_475_143; // 600_851_475_143;
    let max_prime: usize = 10_000_000; // Static for now, increase
    let mut prime_count = 100;
    let mut final_factors: Vec<usize> = Vec::new();

    loop {
        print!(
            "Attempting factorization of using {} primes ... ",
            prime_count
        );
        if attempt_factorization(
            *&number_to_factorize as usize,
            *&max_prime,
            *&prime_count,
            &mut final_factors,
        ) == true
        {
            print!("Success!\n");
            break;
        }
        prime_count += 100;
        print!("Failed\n");
    }
    println!(
        "The factorization of {} is composed of {} numbers
        and is the following: \n {:?}",
        number_to_factorize,
        final_factors.len(),
        final_factors
    );
    let mut prime_found: bool = false;
    {
        let sieve = Sieve::new(max_prime);

        for factor in final_factors.iter().rev() {
            if sieve.is_prime(*factor) {
                println!("And the largest prime factor is {}", *factor);
                prime_found = true;
                break;
            }
        }
    }
    if !prime_found {
        println!("None of the factors are prime!");
    }
}

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

// Single section code

// fn main() {
//     let mut number_to_factorize : usize = 16;//600_851_475_143;
//     let sieve = Sieve::new(10_000_000);
//     let prime_list = (1..=10000).map(|x| sieve.nth_prime(x)).collect::<Vec<_>>();
//     let mut final_factors: Vec<usize> = Vec::new();
//
//     for prime_number in prime_list {
//         loop {
//             println!("Test prime {:?}", prime_number);
//             if number_to_factorize % prime_number == 0 {
//                 println!("Prime factor found {:?}", prime_number);
//                 final_factors.push(prime_number);
//                 number_to_factorize /= prime_number;
//                 println!("Full reduced to {:?}", number_to_factorize);
//             } else {
//                 break;
//             }
//         }
//     }
//     assert_eq!(number_to_factorize, 1);
//     println!("The factorization of {} is composed of {} numbers
//         and is the following: \n {:#?}", number_to_factorize, final_factors.len(), final_factors);
// }
