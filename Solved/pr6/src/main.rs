/* The sum of the squares of the first ten natural numbers is,
    1^2 + 2^2 + ... + 10^2 = 385
  The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)^2 = 552 = 3025
  Hence the difference between the sum of the squares of the first ten natural numbers and the
    square of the sum is 3025 − 385 = 2640.
  Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum. */

fn main() {
    let sum_of_squares = ((1 as i32)..=100).fold(0 as i32, |acc, x| acc + x.pow(2));
    let square_of_sum = ((1 as i32)..=100).sum::<i32>().pow(2);
    println!("The difference between the square of the sum and the sum of squares is {:?}",
        square_of_sum - sum_of_squares);
}
