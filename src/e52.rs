use itertools::Itertools;

pub(crate) fn e52() {
    println!("{}", permuted_multiples())
}

fn permuted_multiples() -> usize {
    let mut n = 1;
    loop {
        let n_2 = to_digits(2 * n);
        let n_3 = to_digits(3 * n);
        let n_4 = to_digits(4 * n);
        let n_5 = to_digits(5 * n);
        let n_6 = to_digits(6 * n);

        if n_2 == n_3 && n_3 == n_4 && n_4 == n_5 && n_5 == n_6 {
            return n;
        }
        n += 1;
    }
}

fn to_digits(mut n: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.iter().sorted().map(|d| *d).collect()
}

#[cfg(test)]
mod e52_tests {
    use crate::e52::permuted_multiples;

    #[test]
    fn permuted_multiples_works() {
        assert_eq!(142857, permuted_multiples());
    }
}
