pub(crate) fn e16() {
    println!("{}", power_digit_sum(1000))
}

fn power_digit_sum(exponential: u32) -> u32 {
    let pow = pow_2(exponential);
    pow.iter().sum()
}

fn pow_2(exponential: u32) -> Vec<u32> {
    let mut ans = vec!{ 1 };
    for _ in 0..exponential {
        let mut new_ans = vec!();
        let mut carryover = 0;
        for i in (0..ans.len()).rev() {
            let dig = ans[i] * 2 + carryover;
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
    ans
}

#[cfg(test)]
mod e16_tests {
    use crate::e16::{pow_2, power_digit_sum};

    #[test]
    fn pow_2_works() {
        assert_eq!(vec!{ 1 }, pow_2(0));
        assert_eq!(vec!{ 2 }, pow_2(1));
        assert_eq!(vec!{ 4 }, pow_2(2));
        assert_eq!(vec!{ 8 }, pow_2(3));
        assert_eq!(vec!{ 1, 6 }, pow_2(4));
        assert_eq!(vec!{ 3, 2 }, pow_2(5));
        assert_eq!(vec!{ 6, 4 }, pow_2(6));
        assert_eq!(vec!{ 1, 2, 8 }, pow_2(7));
    }

    #[test]
    fn power_digit_sum_works() {
        assert_eq!(22, power_digit_sum(14));
        assert_eq!(26, power_digit_sum(15));
    }
}
