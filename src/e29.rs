use std::collections::HashSet;
use crate::utils::toolbox::power_big;

pub(crate) fn e29() {
    println!("{}", distinct_powers())
}

fn distinct_powers() -> usize {
    let mut ans: HashSet<String> = HashSet::new();
    for a in 2..=100 {
        for b in 2..=100 {
            ans.insert(power_big(a, b));
        }
    }
    ans.len()
}

#[cfg(test)]
mod e29_tests {
    use crate::e29::distinct_powers;

    #[test]
    fn distinct_powers_works() {
        assert_eq!(9183, distinct_powers());
    }
}
