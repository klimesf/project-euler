pub(crate) fn e25() {
    println!("{}", calc(1000))
}

fn calc(arg: usize) -> usize {
    let mut prev_prev = vec! { 1 };
    let mut prev = vec! { 1 };
    let mut i = 3;
    loop {
        let curr = sum_big(&prev_prev, &prev);
        if curr.len() >= arg {
            return i;
        }

        i += 1;
        prev_prev = prev;
        prev = curr;
    }
}

fn sum_big(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    let mut ans = vec!();
    let mut carryover = 0;
    for i in 0..a.len().max(b.len()) {
        let ai = if i >= a.len() { 0 } else { a[i] };
        let bi = if i >= b.len() { 0 } else { b[i] };
        let sum = ai + bi + carryover;
        ans.push(sum % 10);
        carryover = sum / 10;
    }

    while carryover > 0 {
        ans.push(carryover % 10);
        carryover /= 10;
    }

    ans
}

#[cfg(test)]
mod e25_tests {
    use crate::e25::{calc, sum_big};

    #[test]
    fn sum_big_works() {
        // The numbers are reversed!
        assert_eq!(vec! {2}, sum_big(&vec! {1}, &vec! {1})); // 1 + 1
        assert_eq!(vec! {0, 1}, sum_big(&vec! {9}, &vec! {1})); // 9 + 1
        assert_eq!(vec! {0, 2}, sum_big(&vec! {9, 1}, &vec! {1})); // 19 + 1
        assert_eq!(vec! {0, 1, 0, 1}, sum_big(&vec! {9, 1}, &vec! {1, 9, 9})); // 19 + 991
        assert_eq!(vec! {0, 1, 0, 0, 1}, sum_big(&vec! {9, 1}, &vec! {1, 9, 9, 9})); // 19 + 9991
    }

    #[test]
    fn calc_works() {
        assert_eq!(4782, calc(1000));
    }
}
