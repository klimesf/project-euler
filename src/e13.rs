use std::fs;

pub(crate) fn e13() {
    println!("{}", multiply_big(fs::read_to_string("input/e13/input.txt").unwrap()))
}

fn multiply_big(input: String) -> String {
    let nums: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let num_len = nums[0].len();

    let mut ans = vec![];
    let mut overflow = 0;
    for i in (0..num_len).rev() {
        let mut sum = overflow;
        for n in 0..nums.len() {
            sum += nums[n][i];
        }
        ans.push(sum % 10);
        overflow = sum / 10;
    }

    while overflow > 0 {
        ans.push(overflow % 10);
        overflow /= 10;
    }

    ans.reverse();
    let mut out = String::new();
    for i in 0..10 {
        out.push_str(format!("{}", ans[i] % 10).as_str());
    }
    out
}

#[cfg(test)]
mod e13_tests {
    use crate::e13::multiply_big;
    use std::fs;

    #[test]
    fn multiply_big_works() {
        assert_eq!("5537376230", multiply_big(fs::read_to_string("input/e13/input.txt").unwrap()));
    }
}
