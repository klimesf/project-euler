use itertools::Itertools;
use crate::utils::toolbox::prime_factors;

pub(crate) fn e47() {
    println!("{}", calc())
}

fn calc() -> usize {
    let prime_factors: Vec<Vec<u32>> = (0..1000000)
        .map(|n| prime_factors(n).iter().unique().map(|n| *n).collect())
        .collect();

    for i in 2..prime_factors.len() - 3 {
        let i1 = &prime_factors[i];
        let i2 = &prime_factors[i + 1];
        let i3 = &prime_factors[i + 2];
        let i4 = &prime_factors[i + 3];

        if i1.len() >= 4 && i2.len() >= 4 && i3.len() >= 4 && i4.len() >= 4 {
            return i;
        }
    }
    panic!()
}

#[cfg(test)]
mod e47_tests {
    use crate::e47::{calc};

    #[test]
    fn calc_works() {
        assert_eq!(134043, calc());
    }
}
