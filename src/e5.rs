use crate::utils::toolbox::lcm_64;
use std::ops::RangeInclusive;

pub(crate) fn e5() {
    println!("{}", smallest_multiple(1..=20))
}

fn smallest_multiple(range: RangeInclusive<i64>) -> i64 {
    let mut ans = 1;
    for i in range {
        ans = lcm_64(ans, i);
    }
    ans
}

#[cfg(test)]
mod e5_tests {
    use crate::e5::smallest_multiple;

    #[test]
    fn calc_works() {
        assert_eq!(2520, smallest_multiple(1..=10));
    }
}
