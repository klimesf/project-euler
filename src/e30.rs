pub(crate) fn e30() {
    println!("{}", digit_fifth_powers())
}

fn digit_fifth_powers() -> usize {
    let mut ans = 0;
    for n in 2..=999999 {
        if is_digits_fifth_power(n) {
            ans += n
        }
    }
    ans
}

fn is_digits_fifth_power(n: usize) -> bool {
    let mut sum = 0;
    let mut rem = n;
    while rem > 0 {
        let i = rem % 10;
        sum += i.pow(5);
        rem /= 10;
    }
    sum == n
}

#[cfg(test)]
mod e30_tests {
    use crate::e30::digit_fifth_powers;

    #[test]
    fn digit_fifth_powers_works() {
        assert_eq!(443839, digit_fifth_powers());
    }
}
