/* In England the currency is made up of pound, £, and pence, p, and there are eight coins
        in general circulation:
    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
It is possible to make £2 in the following way:
    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
How many different ways can £2 be made using any number of coins? */

// Idea.  Starting with the highest denomination, do a reverse decision tree walking down each
//  possible path from highest possible of each coin in decreasing order of denomination value

const TOTAL : u8 = 200;

fn main() {
    let mut comb = Denomination { p1: 0, p2: 0, p5: 0, p10: 0,
        p20: 0, p50: 0, pnd1: 0, pnd2: 0 };
    let mut full_combs : Vec<Denomination> = Vec::new();

    // Walking from most valuable denomination to least, find all possible combinations
    'pnd2_loop: for pnd2_count in (0u8..(TOTAL % DenomWorth::pnd2.value()) as u8).rev() {
        comb.pnd2 = pnd2_count;
        if comb.remaining_worth() == 0 {
            full_combs.push(comb.clone());
            comb.reset();
            continue 'pnd2_loop;
        }
        'pnd1_loop: for pnd1_count in (0u8..(TOTAL % DenomWorth::pnd1.value()) as u8).rev() {
            comb.pnd1 = pnd1_count;
            if comb.remaining_worth() == 0 {
                full_combs.push(comb.clone());
                comb.reset();
                continue 'pnd1_loop;
            }
            'p50_loop: for p50_count in (0u8..(TOTAL % DenomWorth::p50.value()) as u8).rev() {
                comb.p50 = p50_count;
                if comb.remaining_worth() == 0 {
                    full_combs.push(comb.clone());
                    comb.reset();
                    continue 'p50_loop;
                }
                'p20_loop: for p20_count in (0u8..(TOTAL % DenomWorth::p20.value()) as u8).rev() {
                    comb.p20 = p20_count;
                    if comb.remaining_worth() == 0 {
                        full_combs.push(comb.clone());
                        comb.reset();
                        continue 'p20_loop;
                    }
                    'p10_loop: for p10_count in (0u8..(TOTAL % DenomWorth::p10.value()) as u8).rev() {
                        comb.p10 = p10_count;
                        if comb.remaining_worth() == 0 {
                            full_combs.push(comb.clone());
                            comb.reset();
                            continue 'p10_loop;
                        }
                        'p5_loop: for p5_count in (0u8..(TOTAL % DenomWorth::p5.value()) as u8).rev() {
                            comb.p5 = p5_count;
                            if comb.remaining_worth() == 0 {
                                full_combs.push(comb.clone());
                                comb.reset();
                                continue 'p5_loop;
                            }
                            'p2_loop: for p2_count in (0u8..(TOTAL % DenomWorth::p2.value()) as u8).rev() {
                                comb.p2 = p2_count;
                                if comb.remaining_worth() == 0 {
                                    full_combs.push(comb.clone());
                                    comb.reset();
                                    continue 'p2_loop;
                                }
                                'p1_loop: for p1_count in (0u8..(TOTAL % DenomWorth::p1.value()) as u8).rev() {
                                    comb.p1 = p1_count;
                                    if comb.remaining_worth() == 0 {
                                        full_combs.push(comb.clone());
                                        comb.reset();
                                        continue 'p1_loop;
                                    }



                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:#?}", comb);
}

#[derive(Debug)]
#[derive(Clone)]
struct Denomination {
    p1 : u8,
    p2 : u8,
    p5 : u8,
    p10 : u8,
    p20 : u8,
    p50 : u8,
    pnd1 : u8,
    pnd2 : u8,
}

impl Denomination {
    fn remaining_worth(&self) -> u8 {
        (TOTAL
            - self.pnd2 * DenomWorth::pnd2.value()
            - self.pnd1 * DenomWorth::pnd1.value()
            - self.p50 * DenomWorth::p50.value()
            - self.p20 * DenomWorth::p20.value()
            - self.p10 * DenomWorth::p10.value()
            - self.p5 * DenomWorth::p5.value()
            - self.p2 * DenomWorth::p2.value()
            - self.p1 * DenomWorth::p1.value())
    }
    fn reset(&mut self) {
        self.p1 = 0;
        self.p2 = 0;
        self.p5 = 0;
        self.p10 = 0;
        self.p20 = 0;
        self.p50 = 0;
        self.pnd1 = 0;
        self.pnd2 = 0;
    }
}

#[allow(non_camel_case_types)]
enum DenomWorth {
    p1,
    p2,
    p5,
    p10,
    p20,
    p50,
    pnd1,
    pnd2,
}

impl DenomWorth {
    fn value(&self) -> u8 {
        match *self {
            DenomWorth::p1 => 1,
            DenomWorth::p2 => 2,
            DenomWorth::p5 => 5,
            DenomWorth::p10 => 10,
            DenomWorth::p20 => 20,
            DenomWorth::p50 => 50,
            DenomWorth::pnd1 => 100,
            DenomWorth::pnd2 => 200,
        }
    }
}
