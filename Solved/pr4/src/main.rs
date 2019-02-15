/* A palindromic number reads the same both ways.
The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers. */

struct GreatestPair {
    width: i32,
    height: i32,
    product: i32,
}

fn main() {
    let mut product_str = String::with_capacity((9999 * 9999).to_string().len());
    let mut max_so_far: GreatestPair = GreatestPair {
        width: 0,
        height: 0,
        product: 0,
    };
    'outerloop: for outer_num in (1000..10000).rev() {
        'innerloop: for inner_num in (outer_num..10000).rev() {
            let product_num = outer_num * inner_num;
            product_str.push_str(&product_num.to_string());
            // println!("Testing product: {:?}", product_num);
            if is_palindrome(&product_str) {
                if product_num > max_so_far.product {
                    max_so_far.width = outer_num;
                    max_so_far.height = inner_num;
                    max_so_far.product = product_num;
                }
            }
            product_str.clear();
        }
    }
    println!(
        "The largest palindrome occurring as the product of two three digit numbers
        is {product_num} from {outer_num} and {inner_num}",
        product_num = max_so_far.product,
        outer_num = max_so_far.width,
        inner_num = max_so_far.height
    );
}

fn is_palindrome(num_in: &str) -> bool {
    // An inefficient, but effective method
    let half = num_in.len() / 2;
    // Can do a byte iteration because we are working with ascii characters, in general with utf-8
    // this fails and a char comparison must be done
    num_in
        .bytes()
        .take(half)
        .eq(num_in.bytes().rev().take(half))
}
