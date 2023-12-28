use std::fs;

pub(crate) fn e8() {
    println!("{}", largest_product(fs::read_to_string("input/e8/input.txt").unwrap(), 13))
}

fn largest_product(input: String, arg: usize) -> u64 {
    let digits: Vec<u64> = input.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();

    let mut max = 0;
    for i in 0..digits.len() - arg {
        let mut prd = 1;
        for j in 0..arg {
            prd *= digits[i + j];
        }
        max = max.max(prd);
    }
    max
}

#[cfg(test)]
mod e8_tests {
    use std::fs;
    use crate::e8::{largest_product};

    #[test]
    fn calc_works() {
        assert_eq!(5832, largest_product(fs::read_to_string("input/e8/input.txt").unwrap(), 4));
    }
}
