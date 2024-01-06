use itertools::Itertools;
use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e41() {
    println!("{}", calc())
}

fn calc() -> usize {
    let sieve = sieve_of_eratosthenes(7654321);

    for i in (0..=7654321).rev() {
        if !sieve[i] { continue }
        if is_pandigital(i) { return i }
    }
    panic!("no pandigital prime found");
}

fn is_pandigital(n: usize) -> bool {
    let digs: Vec<char> = format!("{}", n).chars().collect();
    let digs_cnt = digs.iter().counts_by(|c| c.to_digit(10).unwrap() as usize);
    (1..=digs.len()).all(|i| *digs_cnt.get(&i).unwrap_or(&0) == 1)
}

#[cfg(test)]
mod e41_tests {
    use crate::e41::{calc, is_pandigital};

    #[test]
    fn is_pandigital_works() {
        assert_eq!(true, is_pandigital(123456789));
        assert_eq!(false, is_pandigital(12345679));
        assert_eq!(true, is_pandigital(2143));
        assert_eq!(false, is_pandigital(2144));
        assert_eq!(false, is_pandigital(2145));
    }

    #[test]
    fn calc_works() {
        assert_eq!(7652413, calc());
    }
}
