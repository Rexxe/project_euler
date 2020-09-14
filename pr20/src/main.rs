/*
--------------------------------- Problem Statement ---------------------------------
    n! means n × (n − 1) × ... × 3 × 2 × 1
    For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
     and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
    Find the sum of the digits in the number 100!
-------------------------------------------------------------------------------------
------------------------------- Structure of Solution -------------------------------
In short, 100! is massive (157 digits) and while it is entirely possible to do so,
we seek to avoid calculating and storing it in pursuit of a generalized algorithm which
will also work for indefinitely larger numbers...
To do so, we propose the following:

Say our number, 100!, is known as 'N' and is composed of digits a_1 through a_n:
We are thus looking for the sum of the digits: a_1 + ... + a_n = S. We wish to determin
the value of S. To do so, we need only determine the value of a_1, ..., a_n.
For any number M, the affect of multiplication on the sum of its digits b_1 + ... + b_m = T
can be determined by the effect of the multiplication on each individual digit, thus:
    b_1 + ... + b_m = T
    X(b_1 + ... + b_m) = X(T)
    Xb_1 + ... + Xb_m = XT.
Indicating that the effect on the overall sum is the same as the multiplier's (X, here)
effect on the digits, perhaps an obvious conclusion.  The number itself, can, however,
be expressed likewise:
   b_1*10^(m-1) + ... + b_m * 10^0 = M.
Therefore, the effect on T - the sum of M's digits - is directly related to the factors
of the digits itself.  Hence if we find how each factor within the prime factorization
affects digits and the perform said string of manipulations subsequently across the
ever-increasing array of digits, we have all of the digits within M and hence
the value of the sum of the difference T


-------------------------------------------------------------------------------------
*/

fn main() {



    println!("Hello, world!");
}
