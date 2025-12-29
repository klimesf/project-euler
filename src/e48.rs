use crate::utils::toolbox::power_big;

pub(crate) fn e48() {
    println!("{}", calc())
}

fn calc() -> String {
    let mut ans: Vec<u32> = vec![0; 10];
    for n in 1..=1000 {
        let power: Vec<u32> = power_big(n, n).chars().rev().map(|c| c.to_digit(10).unwrap()).collect();

        let mut carryover = 0;
        for i in 0..10 {
            let pow_i = if i >= power.len() { 0 } else { power[i] };
            let sum = carryover + ans[i] + pow_i;
            ans[i] = sum % 10;
            carryover = sum / 10;
        }
    }

    let mut out = String::new();
    for i in (0..ans.len()).rev() {
        out.push_str(format!("{}", ans[i]).as_str());
    }
    out
}

#[cfg(test)]
mod e48_tests {
    use crate::e48::calc;

    #[test]
    fn calc_works() {
        assert_eq!("9110846700".to_string(), calc());
    }
}
