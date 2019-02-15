// The four adjacent digits in the 1000-digit number that have the greatest product are
//  9 × 9 × 8 × 9 = 5832.

mod sep {
    pub const NUMBER_TO_PROCESS: &str = "73167176531330624919225119674426574742355349194934\
                                         96983520312774506326239578318016984801869478851843\
                                         85861560789112949495459501737958331952853208805511\
                                         12540698747158523863050715693290963295227443043557\
                                         66896648950445244523161731856403098711121722383113\
                                         62229893423380308135336276614282806444486645238749\
                                         30358907296290491560440772390713810515859307960866\
                                         70172427121883998797908792274921901699720888093776\
                                         65727333001053367881220235421809751254540594752243\
                                         52584907711670556013604839586446706324415722155397\
                                         53697817977846174064955149290862569321978468622482\
                                         83972241375657056057490261407972968652414535100474\
                                         82166370484403199890008895243450658541227588666881\
                                         16427171479924442928230863465674813919123162824586\
                                         17866458359124566529476545682848912883142607690042\
                                         24219022671055626321111109370544217506941658960408\
                                         07198403850962455444362981230987879927244284909188\
                                         84580156166097919133875499200524063689912560717606\
                                         05886116467109405077541002256983155200055935729725\
                                         71636269561882670428252483600823257530420752963450";
}

/* Find the thirteen adjacent digits in the 1000-digit number that have the greatest product.
What is the value of this product? */

extern crate fine_grained;

use fine_grained::Stopwatch;

fn main() {
    let mut stopwatch = Stopwatch::start_new();
    // Split into non-zero sections
    let mut non_zero_sections: Vec<String> = String::from(sep::NUMBER_TO_PROCESS)
        .split('0')
        .map(|s| String::from(s))
        .collect::<Vec<String>>(); // Fix this hard-casting later? not happy!
                                   // Discard any sections of length less than 13
    non_zero_sections.retain(|s| s.len() >= 13);
    // Walk through each section's sub-sections of 13 bytes (which, since we are dealing with
    // as string of numbers in u8, are equivalent to digits)
    let mut max_product_so_far = 0;
    let mut greatest_combination_so_far = 0;
    let mut combination_id = 0;
    for section in non_zero_sections {
        println!("Inspecting section {}", section);
        for byte_pos in 0..=(section.len() - 13) {
            let subsection: Vec<u8> = section
                .as_bytes()
                .iter()
                .skip(byte_pos)
                .take(13)
                .map(|b| (*b as char).to_digit(10).unwrap())
                .map(|x| x as u8)
                .collect::<Vec<_>>();
            let sum_subsection = subsection
                .clone()
                .iter()
                .fold(1 as u64, |prod: u64, &b| prod * b as u64);
            if sum_subsection > max_product_so_far {
                max_product_so_far = sum_subsection;
                greatest_combination_so_far = combination_id
            }

            println!(
                "{comb_ID:<3} : Number beginning at pos {index:>2} is \
                 {full_subsection} which products to: \
                 {translated_val}",
                comb_ID = combination_id,
                index = byte_pos,
                translated_val = sum_subsection,
                full_subsection = {
                    let mut final_str: String = String::new();
                    // Ascii begins numbers at 48, so displace these then conver to char
                    for &iter_str in subsection.iter() {
                        final_str.push_str(std::str::from_utf8(&vec![iter_str + 48]).unwrap());
                    }
                    final_str
                },
            );
            combination_id += 1;
        }
    }
    // std::thread::sleep(std::time::Duration::from_millis(1000));
    println!(
        "The greatest product of 13 consecutive numbers \
         occurs in row {} in the above sequence is {} and took {} to find",
        greatest_combination_so_far, max_product_so_far, stopwatch
    );

    stopwatch.stop();
}
