// let mut cur_pos : usize = number_list.len() - 1;
// let mut nums_to_remove : Vec<u32> = Vec::new();
// loop {
//     for i in (0..cur_pos).rev() {
//         if number_list[cur_pos] % number_list[i] == 0 {
//             nums_to_remove.push(number_list[i]);
//         }
//     }
//     cur_pos -= match nums_to_remove.len() {
//         0 => 1,
//         _ => nums_to_remove.len(),
//     };
//     number_list.retain(|num| !nums_to_remove.contains(num));
//     nums_to_remove.clear();
//     if cur_pos == 0 { break; }
// }
// // Determine and remove fully sub-comprised factors.  e.g. 12 is fully sub-comprised because of
// //  the inclusion of 16 and 18 (contains 4, 3 respectively)
// // First, build a full list of factorizations
// let mut all_prime_factors : Vec<u32> = Vec::new();
// let mut temp_factor_list : Vec<u32>;
// for num in number_list {
//     temp_factor_list = factorize(&num, &1_000)
//         .iter() // Obtain an iterator
//         .map(|&e| e as u32) // Remap the usize result as u32
//         .filter(|e| !all_prime_factors.contains(e) && num != e) // We only want prime factors that aren't present already
//         .collect::<Vec<u32>>(); // Bring into a Vec<u32>
//     all_prime_factors.append(&mut temp_factor_list);
//     temp_factor_list.clear();
// }
// all_prime_factors.sort(); // Why the hell are you reading this one?
// all_prime_factors.dedup(); // Removes consecutive duplicates; all duplicates since it is sorted
// println!("All prime factors: {:?}", all_prime_factors);
// // Now find those numbers whose factorizations are composed entirely of this set
