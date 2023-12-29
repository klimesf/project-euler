pub(crate) fn e21() {
    println!("{}", amicable_numbers_sum(10000))
}

fn amicable_numbers_sum(to: usize) -> usize {
    let mut d_ans = vec!{ 0; to + 1 };

    let mut ans = 0;
    for i in 1..=to {
        let d = d(i);
        d_ans[i] = d;
        if d < d_ans.len() && d_ans[d] == i && d != i {
            ans += i + d;
        }
    }
    ans
}

fn d(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod e21_tests {
    use crate::e21::{amicable_numbers_sum, d};

    #[test]
    fn d_works() {
        assert_eq!(284, d(220));
        assert_eq!(220, d(284));
    }

    #[test]
    fn amicable_numbers_sum_works() {
        assert_eq!(0, amicable_numbers_sum(10000));
    }
}
