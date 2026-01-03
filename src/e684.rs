#[allow(unused)]
pub(crate) fn e684() {
    println!("{}", inverse_digit_sum())
}

fn inverse_digit_sum() -> u128 {
    let mut fibb = vec![0_u128; 91];
    fibb[0] = 1;
    fibb[1] = 2;
    for i in 2..=90 {
        fibb[i] = fibb[i - 2] + fibb[i - 1];
    }

    let mut ans = 0;
    for i in 2..=90 {
        println!("{} ... {}", i, fibb[i]);
        ans = (ans + big_s(fibb[i])) % 1_000_000_007;
    }
    ans
}

fn small_s(mut n: u128) -> u128 {
    let mut ans = String::new();
    while n > 9 {
        ans.push('9');
        n -= 9;
    }
    ans.push_str(n.to_string().as_str());
    println!("{} ... {}", n, ans);
    ans.chars().rev().collect::<String>().parse().unwrap()
}

fn big_s(k: u128) -> u128 {
    let mut ans = 0;
    for n in 1..=k {
        ans = (ans + small_s(n)) % 1_000_000_007;
    }
    ans
}

#[cfg(test)]
mod e684_tests {
    use crate::e684::{big_s, inverse_digit_sum, small_s};

    #[test]
    fn small_s_works() {
        assert_eq!(1, small_s(1));
        assert_eq!(19, small_s(10));
        assert_eq!(29, small_s(11));
        assert_eq!(299, small_s(20));
        assert_eq!(3999, small_s(30));
        assert_eq!(49999, small_s(40));
    }

    #[test]
    fn big_s_works() {
        assert_eq!(1074, big_s(20));
        assert_eq!(1074, big_s(377));
    }

    #[test]
    fn inverse_digit_sum_works() {
        assert_eq!(0, inverse_digit_sum());
    }
}
