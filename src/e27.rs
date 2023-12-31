use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e27() {
    println!("{}", quadratic_primes())
}

fn quadratic_primes() -> i32 {
    let sieve = sieve_of_eratosthenes(1000000);

    let mut max_consecutive = 0;
    let mut ans = 0;
    for a in -999_i32..=999 {
        for b in -1000_i32..=1000 {
            let mut n: i32 = 0;
            while n.pow(2) + a * n + b >= 2 && sieve[(n.pow(2) + a * n + b) as usize] {
                n += 1;
            }
            if n > max_consecutive {
                max_consecutive = n;
                ans = a * b;
            }
        }
    }

    ans
}

#[cfg(test)]
mod e27_tests {
    use crate::e27::{quadratic_primes};

    #[test]
    fn quadratic_primes_works() {
        assert_eq!(-59231, quadratic_primes());
    }
}
