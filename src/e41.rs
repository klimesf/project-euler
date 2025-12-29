use crate::utils::toolbox::{is_pandigital, sieve_of_eratosthenes};

pub(crate) fn e41() {
    println!("{}", calc())
}

fn calc() -> usize {
    let sieve = sieve_of_eratosthenes(7654321);

    for i in (0..=7654321).rev() {
        if !sieve[i] {
            continue;
        }
        if is_pandigital(i) {
            return i;
        }
    }
    panic!("no pandigital prime found");
}

#[cfg(test)]
mod e41_tests {
    use crate::e41::calc;

    #[test]
    fn calc_works() {
        assert_eq!(7652413, calc());
    }
}
