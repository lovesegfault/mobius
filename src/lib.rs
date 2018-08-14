extern crate num;

use std::ops::{AddAssign, DivAssign};
use num::{Unsigned, Integer, FromPrimitive, CheckedAdd};
use num::integer::{self, Roots};

// TODO: Use const fn when it is stabilized
// c.f. https://github.com/rust-lang/rust/issues/24111
macro_rules! gen {
    ($x: expr) => {T::from_usize($x).unwrap()};
}

pub fn mu<T>(mut x: T) -> isize
where T: Unsigned + Integer + Roots + FromPrimitive
        + Clone + CheckedAdd + AddAssign + DivAssign
{
    let zero = gen!(0);
    let one = gen!(1);
    let two = gen!(2);
    let three = gen!(3);

    let mut prime_count: T = zero.clone();
    // Handle 2 separately
    if x.clone() % two.clone()  == zero {
        x /= two.clone();
        prime_count += one.clone();
        // If 2^2 also divides x
        if x.clone() % two.clone() == zero {
            return 0;
        }
    }

    // Check remaining prime factors
    for i in num::range_step(three, integer::sqrt(x.clone()), two.clone()) {
        // If i is a divisor of x
        if x.clone() % i.clone() == zero {
            x /= i.clone();
            prime_count += one.clone();

            // if i^2 is also a divisor of x
            if x.clone() % i == zero {
                return 0;
            }
        }
    }
    if prime_count % two == zero {
        -1
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
