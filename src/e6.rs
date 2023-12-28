use std::ops::RangeInclusive;

pub(crate) fn e6() {
    println!("{}", sum_square_difference(1..=100))
}

fn sum_square_difference(range: RangeInclusive<usize>) -> usize {
    let sum_of_squares = range.clone().map(|n| n.pow(2)).sum::<usize>();
    let square_of_sum = range.clone().sum::<usize>().pow(2);
    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod e6_tests {
    use crate::e6::{sum_square_difference};

    #[test]
    fn sum_square_difference_works() {
        assert_eq!(2640, sum_square_difference(1..=10));
    }
}
