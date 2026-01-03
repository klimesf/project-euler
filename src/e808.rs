use crate::utils::toolbox::sieve_of_eratosthenes;
use std::collections::HashSet;

#[allow(unused)]
pub(crate) fn e808() {
    println!("{}", reversible_prime_squares())
}

fn reversible_prime_squares() -> usize {
    let max = 100_000_000;
    let sieve = sieve_of_eratosthenes(max);
    let mut squares = HashSet::new();
    for n in 0..max {
        if !sieve[n] {
            continue;
        }
        let square = n * n;
        squares.insert(square);
    }

    let mut ans = 0;
    let mut cnt = 0;
    for n in 0..max {
        if !sieve[n] {
            continue;
        }
        let square = n * n;
        let reverse = reverse(square);
        if square == reverse {
            continue; // is palindrome
        }
        if !squares.contains(&reverse) {
            continue; // reverse is not a square of a prime
        }

        ans += square;
        cnt += 1;
        if cnt == 50 {
            break;
        }
    }

    debug_assert_eq!(50, cnt);
    ans
}

fn reverse(n: usize) -> usize {
    let mut tmp = n;
    let mut reversed = 0;
    while tmp > 0 {
        reversed = reversed * 10 + tmp % 10;
        tmp /= 10;
    }
    reversed
}

#[cfg(test)]
mod e808_tests {
    use crate::e808::reversible_prime_squares;

    #[test]
    fn reversible_prime_squares_works() {
        assert_eq!(3807504276997394, reversible_prime_squares());
    }
}
