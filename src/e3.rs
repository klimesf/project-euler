pub(crate) fn e3() {
    println!("{}", largest_prime_factor(600851475143))
}

fn largest_prime_factor(n: usize) -> usize {
    let mut factors = prime_factors(n);
    factors.sort();
    *factors.last().unwrap()
}

fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut ans = vec!();
    while n % 2 == 0 {
        ans.push(2);
        n = n / 2;
    }

    let mut i = 3;
    loop {
        if i * i > n { break; }
        while n % i == 0 {
            ans.push(i);
            n = n / i;
        }
        i += 2;
    }

    if n > 2 { ans.push(n); }
    ans
}

#[cfg(test)]
mod e3_tests {
    use crate::e3::{largest_prime_factor};

    #[test]
    fn largest_prime_factor_works() {
        assert_eq!(29, largest_prime_factor(13195));
    }
}
