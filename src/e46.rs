use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e46() {
    println!("{}", goldbach_odd_conjecture())
}

fn goldbach_odd_conjecture() -> usize {
    let max_n = 1000000;
    let sieve_eratosthenes = sieve_of_eratosthenes(max_n);
    let sieve_squares = sieve_of_squares(max_n);

    'outer: for n in 4..max_n {
        if n % 2 != 1 { continue } // Not odd
        if sieve_eratosthenes[n] { continue } // Not a composite number

        // find sum of a prime and twice a square
        for prime in (2..n).rev() {
            if !sieve_eratosthenes[prime] { continue } // not a prime

            for square in (0..n - prime).rev() {
                if !sieve_squares[square] { continue } // not a square
                if n == prime + 2 * square {
                    continue 'outer
                }
            }
        }

        // if not found, we got our result
        return n
    }
    panic!("no luck")
}

fn sieve_of_squares(n: usize) -> Vec<bool> {
    let mut sieve = vec!{ false; n + 1 };
    for i in 0..=(n as f64).sqrt() as usize {
        sieve[i.pow(2)] = true;
    }
    sieve
}

#[cfg(test)]
mod e46_tests {
    use crate::e46::{goldbach_odd_conjecture};

    #[test]
    fn goldbach_odd_conjecture_works() {
        assert_eq!(5777, goldbach_odd_conjecture());
    }
}
