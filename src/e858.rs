use itertools::Itertools;
use crate::utils::toolbox::lcm_64;

#[allow(unused)]
pub(crate) fn e858() {
    println!("{}", calc(800))
}

fn calc(n: u64) -> i64 {
    let nums: Vec<i64> = (1..=n).map(|n| n as i64).collect();

    let mut ans = 0;
    for k in 0..=nums.len() {
        println!("{}", k);
        ans += nums.iter().combinations(k).map(|subset| {
            let mut lcm = 1;
            for s in subset {
                lcm = lcm_64(lcm, *s);
            }
            lcm
        }).sum::<i64>()
    }
    ans
}

#[cfg(test)]
mod e858_tests {
    use crate::e858::{calc};

    #[test]
    fn calc_works() {
        assert_eq!(528, calc(5));
        assert_eq!(8463108648960, calc(20));
    }
}
