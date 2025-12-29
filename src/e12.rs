use crate::utils::toolbox::prime_factors;
use std::collections::HashMap;

pub(crate) fn e12() {
    println!("{}", calc(500))
}

fn calc(divisor_cnt_over: u32) -> u32 {
    let mut next = 2;
    let mut triangular = 1;

    loop {
        let prime_factors = prime_factors(triangular);
        let prime_factors_pows = prime_factors.iter().fold(HashMap::new(), |mut acc, factor| {
            *acc.entry(factor).or_insert(0) += 1;
            acc
        });
        let divisor_ctr: u32 = prime_factors_pows.values().map(|pow| *pow + 1).product();
        if divisor_ctr > divisor_cnt_over {
            return triangular;
        }

        triangular += next;
        next += 1;
    }
}

#[cfg(test)]
mod e12_tests {
    use crate::e12::calc;

    #[test]
    fn calc_works() {
        assert_eq!(28, calc(5));
    }
}
