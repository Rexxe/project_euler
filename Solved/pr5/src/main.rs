extern crate bigint;
extern crate primal;
extern crate stopwatch;

mod factorize;

use stopwatch::Stopwatch;

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//  What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    let initial_factor = 2;
    let mut final_factor = 10;

    loop {
        let mut unique_factor_list: Vec<u32> = Vec::new();
        let mut temp_factor_list: Vec<u32>;
        let mut uf_count: u8;
        let mut tf_count: u8;
        let mut i: u8;
        let mut temp_factor: &u32;

        let sw = Stopwatch::start_new();

        for num in (initial_factor..=final_factor).rev() {
            temp_factor_list = factorize(&num, &1_000)
                .iter()
                .map(|&e| e as u32)
                .collect::<Vec<u32>>();
            temp_factor_list.sort();
            // For each unique factor, ensure at least that many exist in the unique_factor_list
            //  If they do not, make up the difference
            i = 0;
            loop {
                // Count of element in temp_factor
                temp_factor = temp_factor_list.get(i as usize).unwrap();
                tf_count = temp_factor_list
                    .iter()
                    .filter(|&x| x == temp_factor)
                    .count() as u8;
                uf_count = unique_factor_list
                    .iter()
                    .filter(|&x| x == temp_factor)
                    .count() as u8;
                // Push the missing instances of the temp_factor onto the uf list
                // println!("{}/{}", tf_count, uf_count);
                // println!("Length of temp: {}. Iterate: {}", temp_factor_list.len(), i);
                for _ in 0..(tf_count as i8 - uf_count as i8) {
                    unique_factor_list.push(*temp_factor);
                }

                i += if tf_count as i8 - uf_count as i8 <= 1 {
                    1
                } else {
                    tf_count - uf_count
                };

                if i >= temp_factor_list.len() as u8 {
                    break;
                }
            }
        }
        unique_factor_list.sort();
        let final_result_arbitrarilylarge = bigint::U512::from(
            unique_factor_list
                .iter()
                .map(|&x| bigint::U512::from(x))
                .fold(bigint::U512::from(1), |acc, x| acc * x),
        );
        assert_eq!(
            verify_divisibility(
                &final_result_arbitrarilylarge,
                &initial_factor,
                &final_factor
            ),
            true
        );

        println!("The full unique factor list was found to be the following: {:?} \
            which means the lowest number containing numbers ranging {}..={} is {}.  This calculation took \
            {}ms",
            unique_factor_list,
            initial_factor,
            final_factor,
            final_result_arbitrarilylarge,
            sw.elapsed_ms());

        final_factor += 1;
    }
}

fn factorize(&number_to_factorize: &u32, &max_prime: &usize) -> Vec<usize> {
    // let max_prime : usize = 1000; // Static for now
    let mut prime_count = 20;
    let mut final_factors: Vec<usize> = Vec::new();
    loop {
        if factorize::attempt_factorization(
            *&number_to_factorize as usize,
            *&max_prime,
            *&prime_count,
            &mut final_factors,
        ) == true
        {
            break;
        }
        prime_count += 20;
    }
    final_factors
}

fn verify_divisibility(
    &final_result: &bigint::U512,
    &initial_factor: &u32,
    &final_factor: &u32,
) -> bool {
    // let mut test_factor_arbitrarilylarge : bigint::U512;
    // for test_factor in initial_factor..=final_factor {
    //     test_factor_arbitrarilylarge = bigint::U512::from(test_factor);
    //     if final_result % test_factor_arbitrarilylarge != bigint::U512::from(0) {
    //         println!("Modulus of {} and {} comes to {:?}", final_result, test_factor_arbitrarilylarge, final_result % test_factor_arbitrarilylarge);
    //         println!("The factor {:?} failed to fit into {}", test_factor_arbitrarilylarge, final_result);
    //         return false
    //     }
    // }
    // true
    (initial_factor..=final_factor)
        .map(|test_factor| bigint::U512::from(test_factor))
        .all(|test_factor| final_result % test_factor == bigint::U512::from(0))
}

// Raw process - No optimization - Run takes ~20 seconds for 1..20 in debug/ ~2 in release
/* fn main() {
     let sw = Stopwatch::start_new();
     let mut num : u32 = 1;
     'num_inc: loop  {
         num += 1;
         for i in 2..=20 {
             if num % i != 0 {
                 continue 'num_inc;
             }
         }
         break;
     }
     println!("The lowest positive number divisible by whole numbers up to 20 is {:?}", num);
     println!("The calculation took {}ms", sw.elapsed_ms());
} */
