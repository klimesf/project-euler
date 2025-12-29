use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e37() {
    println!("{}", truncatable_primes())
}

fn truncatable_primes() -> usize {
    let sieve = sieve_of_eratosthenes(1000000);

    let mut ans = 0;
    let mut total = 0;
    let mut n = 11; // Do not count 1-digit primes
    while total <= 10 {
        // There are only 11 truncatable primes
        n += 1;

        if !sieve[n] {
            continue;
        }

        let mut truncatable = true;
        let mut n_div = n;
        let mut n_10 = 1;
        while n_div > 0 {
            if !sieve[n_div] {
                truncatable = false;
                break;
            }

            n_div /= 10;
            n_10 *= 10;
        }

        let mut n_rem = n;
        while n_10 > 0 {
            if !sieve[n_rem] {
                truncatable = false;
                break;
            }

            n_rem = n_rem % n_10;
            n_10 /= 10;
        }

        if truncatable {
            total += 1;
            ans += n;
        }
    }
    ans
}

#[cfg(test)]
mod e37_tests {
    use crate::e37::truncatable_primes;

    #[test]
    fn truncatable_primes_works() {
        assert_eq!(748317, truncatable_primes());
    }
}
