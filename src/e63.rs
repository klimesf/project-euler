pub(crate) fn e63() {
    println!("{}", powerful_digit_counts())
}

fn powerful_digit_counts() -> usize {
    (1..10_u128) // n-th power of >= 10 will always have >= n+1 digits
        .map(|base| {
            let mut pow = 1;
            let mut res = base;
            let mut ans = 0;
            loop {
                let log = res.ilog10() as u128 + 1; // Number of digits in a number can be obtained by log 10
                if log == pow {
                    ans += 1;
                } else if log < pow {
                    break; // The log won't grow as fast as the linear pow
                }
                res *= base;
                pow += 1;
            }
            ans
        })
        .sum()
}

#[cfg(test)]
mod e63_tests {
    use crate::e63::powerful_digit_counts;

    #[test]
    fn powerful_digit_counts_works() {
        assert_eq!(49, powerful_digit_counts());
    }
}
