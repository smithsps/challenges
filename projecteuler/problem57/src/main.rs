extern crate num;

use num::bigint::BigUint;

fn main() {
    let mut frac: (BigUint, BigUint) = (BigUint::from(3u32), BigUint::from(2u32));
    let mut time_of_greater_numerator_digits = 0;
    let two = BigUint::from(2u8);

    for _ in 0..10000 {
        if frac.0.to_str_radix(10).len() > frac.1.to_str_radix(10).len() {
            time_of_greater_numerator_digits += 1;
        }
        //println!("{}: {}/{}", i, number_of_digits(&frac.0), number_of_digits(&frac.1));

        frac = (
            frac.1.clone() * two.clone() + frac.0.clone(),
            frac.0 + frac.1,
        )
    }

    println!("{}", time_of_greater_numerator_digits);
}
