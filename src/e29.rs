use std::collections::HashSet;

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

fn power_big(n: usize, exponential: usize) -> String {
    let mut ans = vec!{ 1 };
    for _ in 0..exponential {
        let mut new_ans = vec!();
        let mut carryover = 0;
        for i in (0..ans.len()).rev() {
            let dig = ans[i] * n + carryover;
            new_ans.push(dig % 10);
            carryover = dig / 10;
        }

        while carryover > 0 {
            new_ans.push(carryover % 10);
            carryover /= 10;
        }

        new_ans.reverse();
        ans = new_ans;
    }

    let mut out = String::new();
    for i in 0..ans.len() {
        out.push_str(format!("{}", ans[i] % 10).as_str());
    }
    out
}

#[cfg(test)]
mod e29_tests {
    use crate::e29::{distinct_powers, power_big};

    #[test]
    fn power_big_works() {
        assert_eq!("1", power_big(1, 0));
        assert_eq!("2", power_big(2, 1));
        assert_eq!("4", power_big(2, 2));
        assert_eq!("25", power_big(5, 2));
        assert_eq!("125", power_big(5, 3));
    }

    #[test]
    fn distinct_powers_works() {
        assert_eq!(9183, distinct_powers());
    }
}
