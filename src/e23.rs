pub(crate) fn e23() {
    println!("{}", sum())
}

fn sum() -> usize {
    let mut abundant_nums = vec![];
    for i in 2..=28123 {
        if is_abundant(i) {
            abundant_nums.push(i);
        }
    }

    let mut sieve = vec![false; 28124];
    for i in 0..abundant_nums.len() {
        for j in 0..=i {
            let a = abundant_nums[i];
            let b = abundant_nums[j];
            if a + b > 28123 {
                continue;
            }
            sieve[a + b] = true;
        }
    }

    let mut ans = 0;
    for i in 1..sieve.len() {
        if !sieve[i] {
            ans += i;
        }
    }
    ans
}

fn is_abundant(n: usize) -> bool {
    let mut divisor_sum = 0;
    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            divisor_sum += i;
            if n / i != i {
                divisor_sum += n / i;
            }
        }
    }
    divisor_sum > 2 * n
}

#[cfg(test)]
mod e23_tests {
    use crate::e23::{is_abundant, sum};

    #[test]
    fn is_abundant_works() {
        assert_eq!(false, is_abundant(28));
        assert_eq!(true, is_abundant(12));
    }

    #[test]
    fn sum_works() {
        assert_eq!(4179871, sum());
    }
}
