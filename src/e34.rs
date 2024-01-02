pub(crate) fn e34() {
    println!("{}", calc())
}

fn calc() -> u32 {
    let mut ans = 0;
    for i in 3..1000000 {
        let digit_factorials = digit_factorials(i);
        if digit_factorials == i {
            ans += i;
        }
    }
    ans
}

fn digit_factorials(mut i: u32) -> u32 {
    let mut sum = 0;
    while i != 0 {
        sum += factorial(i % 10);
        i /= 10;
    }
    sum
}

fn factorial(num: u32) -> u32 {
    (1..=num).product()
}

#[cfg(test)]
mod e34_tests {
    use crate::e34::{calc, digit_factorials};

    #[test]
    fn digit_factorials_works() {
        assert_eq!(145, digit_factorials(145));
    }

    #[test]
    fn calc_works() {
        assert_eq!(40730, calc());
    }
}
