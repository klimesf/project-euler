pub(crate) fn e57() {
    println!("{}", square_root_convergents())
}

fn square_root_convergents() -> usize {
    (1..=1000)
        .map(|steps| sqrt_convergent(steps))
        .filter(|(numerator_len, denominator_len)| numerator_len > denominator_len)
        .count()
}

fn sqrt_convergent(steps: usize) -> (usize, usize) {
    let mut numerator = vec![3];
    let mut denominator = vec![2];
    for _ in 1..steps {
        numerator = sum(&numerator, &denominator);
        std::mem::swap(&mut numerator, &mut denominator);
        numerator = sum(&numerator, &denominator);
    }
    (numerator.len(), denominator.len())
}

fn sum(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut ans = vec![];
    ans.reserve(a.len() + 1);

    let mut carryover = 0;
    for i in 0..a.len().max(b.len()) {
        let sum = if i >= b.len() {
            a[i] + carryover
        } else if i >= a.len() {
            b[i] + carryover
        } else {
            a[i] + b[i] + carryover
        };
        ans.push(sum % 10);
        carryover = sum / 10;
    }
    if carryover > 0 {
        ans.push(carryover);
    }
    ans
}

#[cfg(test)]
mod e57_tests {
    use crate::e57::{square_root_convergents, sum};

    #[test]
    fn sum_works() {
        assert_eq!(vec![2, 1], sum(&vec![6], &vec![6]));
        assert_eq!(vec![4, 1, 1], sum(&vec![1, 1, 1], &vec![3]));
        assert_eq!(vec![0, 2, 1], sum(&vec![1, 1, 1], &vec![9]));
        assert_eq!(vec![0, 2, 1], sum(&vec![9], &vec![1, 1, 1]));
        assert_eq!(vec![2, 2, 0, 1], sum(&vec![1, 1, 1], &vec![1, 1, 9]));
    }

    #[test]
    fn square_root_convergents_works() {
        assert_eq!(153, square_root_convergents());
    }
}
