pub(crate) fn e20() {
    println!("{}", factorial_digit_sum(100))
}

fn factorial_digit_sum(n: u32) -> u32 {
    factorial(n).iter().sum()
}

fn factorial(n: u32) -> Vec<u32> {
    let mut ans = vec!{ 1 };
    for multiplier in 2..=n {
        let mut new_ans = vec!();
        let mut carryover = 0;
        for i in (0..ans.len()).rev() {
            let dig = ans[i] * multiplier + carryover;
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
mod e20_tests {
    use crate::e20::{factorial, factorial_digit_sum};

    #[test]
    fn factorial_works() {
        assert_eq!(vec! { 3, 6, 2, 8, 8, 0, 0 }, factorial(10));
    }

    #[test]
    fn factorial_digit_sum_works() {
        assert_eq!(27, factorial_digit_sum(10));
    }
}
