use crate::utils::toolbox::power_big;

pub(crate) fn e56() {
    println!("{}", calc())
}

fn calc() -> u32 {
    let mut max = 0;
    for a in 1..=100 {
        for b in 1..=100 {
            let sum = power_big(a, b).chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum();
            max = max.max(sum);
        }
    }
    max
}

#[cfg(test)]
mod e56_tests {
    use crate::e56::{calc};

    #[test]
    fn calc_works() {
        assert_eq!(972, calc());
    }
}
