/* The number, 197, is called a circular prime because all rotations of the digits:
197, 971, and 719, are themselves prime.
There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
How many circular primes are there below one million? */
extern crate console;
extern crate indicatif;
extern crate permutation;
extern crate primal;
extern crate unicode_segmentation;

use console::Term;
use permutation::Permutation;
use primal::Sieve;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut count = 0;
    let sieve = Sieve::new(1_000_000);
    let term = Term::stdout();
    for i in 1..1_000_000 {
        if sieve.is_prime(i) {
            if is_circular_prime(&(i as i32).to_string(), &sieve) == true {
                count += 1;
                term.write_line(&format!("Current circular # {}", i))
                    .expect("Could not write to terminal");
            }
        }
    }
    term.write_line(&format!(
        "There are {} circular primes below 1 million",
        count
    ))
    .expect("Could not write to terminal");
}

fn obtain_rotations<'a>(vec_to_rotate: &'a Vec<&str>, upper_bound: usize) -> Vec<Vec<&'a str>> {
    let mut all_rotations: Vec<Vec<&str>> = Vec::new();
    let mut one_rotation: Vec<&str>;
    let mut one_rotation_positions: Vec<usize>;
    let mut permutation: Permutation;

    // Outer loop is both the initial enumeration position and the (n+1)th rotation
    let mut cur_pos: usize;
    for i in (0 as usize)..=(upper_bound as usize) {
        cur_pos = i;
        one_rotation_positions = Vec::new();
        // Inner loop to form each rotation
        for _ in 0..=upper_bound {
            one_rotation_positions.push(cur_pos);
            cur_pos += 1;
            if cur_pos > upper_bound {
                cur_pos = 0;
            }
        }
        permutation = Permutation::from_vec(one_rotation_positions);
        one_rotation = permutation.apply_slice(&vec_to_rotate[..]);
        all_rotations.push(one_rotation.clone());
    }
    all_rotations
}

fn is_circular_prime(num_in: &str, sieve: &primal::Sieve) -> bool {
    // Break into a Vec<&str>
    let digit_vector: Vec<&str> =
        UnicodeSegmentation::graphemes(num_in, true).collect::<Vec<&str>>();
    // Form all the rotations
    let rotations: Vec<Vec<&str>> = obtain_rotations(&digit_vector, digit_vector.len() - 1);
    // Iterate across all rotations
    let mut new_number = String::with_capacity(7);
    for rotation in rotations {
        // Recombine the digits into a number
        for &digit in rotation.iter() {
            new_number.push_str(digit);
        }

        // Check that it is prime
        if !sieve.is_prime((&new_number).parse::<usize>().unwrap()) {
            return false;
        }
        new_number.clear();
    }
    true
}
