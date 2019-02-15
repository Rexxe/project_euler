/* A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
     a2 + b2 = c2
   For example, 32 + 42 = 9 + 16 = 25 = 52.
   There exists exactly one Pythagorean triplet for which a + b + c = 1000.
   Find the product abc. */
// Initial inequality implies c >= 335

fn main() {
    let mut c : u32 = 335;
    let mut b : u32 = 0;
    let mut a : u32 = 0;
    let mut pythag_found : bool = false;

    'loop_c : loop {
        let mut b_values = ((1 as u32)..(c-1)).rev();
        'loop_b : loop {
            match b_values.next() {
                Some(val) => b = val,
                None => break 'loop_b,
            };
            let mut a_values = ((1 as u32)..(b-1)).rev();
            'loop_a : loop {
                match a_values.next() {
                    Some(val) => a = val,
                    None => break 'loop_a,
                };
                if a.pow(2) + b.pow(2) == c.pow(2) && (a + b + c == 1000) {
                    pythag_found = true;
                    break 'loop_c;
                }
            }
        }
        // println!("Case c={} failed to produce triplet", c);
        c += 1;
    }
    match pythag_found {
        false => println!("No Pythag triplet was found to match the criteria"),
        true => {
            println!("The final resulting Pythag Triplet is ({a},{b},{c})",
                a=a,b=b,c=c);
            println!("The final product thus is {}", (a as u64)*(b as u64)*(c as u64));
        },
    }
}
