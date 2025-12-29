use crate::utils::toolbox::gcd;

pub(crate) fn e33() {
    println!("{}", digit_cancelling_fractions())
}

fn digit_cancelling_fractions() -> i32 {
    let mut numerator = 1;
    let mut denominator = 1;

    for i in 10..=99 {
        for j in i + 1..=99 {
            let a = to_digs(i);
            let b = to_digs(j);

            if a[1] == b[0]
                && a[1] != '0'
                && a[0].to_digit(10).unwrap() as f32 / b[1].to_digit(10).unwrap() as f32 == i as f32 / j as f32
            {
                numerator *= i;
                denominator *= j;
            }
        }
    }

    let gcd = gcd(numerator as i32, denominator as i32);

    denominator as i32 / gcd
}

fn to_digs(mut n: u32) -> Vec<char> {
    let mut ans = vec![];
    while n > 0 {
        ans.push(char::from_digit(n % 10, 10).unwrap());
        n /= 10;
    }
    ans.reverse();
    ans
}

#[cfg(test)]
mod e33_tests {
    use crate::e33::digit_cancelling_fractions;

    #[test]
    fn digit_cancelling_fractions_works() {
        assert_eq!(100, digit_cancelling_fractions());
    }
}
