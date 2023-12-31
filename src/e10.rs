use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e10() {
    println!("{}", calc(2000000))
}

fn calc(n: usize) -> usize {
    let sieve = sieve_of_eratosthenes(n);
    let mut ans = 0;
    for i in 2..=n {
        if sieve[i] { ans += i }
    }
    ans
}

#[cfg(test)]
mod e10_tests {
    use crate::e10::{calc};

    #[test]
    fn calc_works() {
        assert_eq!(17, calc(10));
    }
}
