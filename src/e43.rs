use itertools::Itertools;

pub(crate) fn e43() {
    println!("{}", calc())
}

fn calc() -> usize {
    let digs = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    digs.iter().permutations(digs.len())
        .filter(|perm| *perm[0] != '0')
        .filter(|perm| is_substring_divisible(perm))
        .map(|perm| to_usize(&*perm))
        .sum()
}

fn is_substring_divisible(perm: &Vec<&char>) -> bool {
    to_usize(&perm[1..4]) % 2 == 0
        && to_usize(&perm[2..5]) % 3 == 0
        && to_usize(&perm[3..6]) % 5 == 0
        && to_usize(&perm[4..7]) % 7 == 0
        && to_usize(&perm[5..8]) % 11 == 0
        && to_usize(&perm[6..9]) % 13 == 0
        && to_usize(&perm[7..10]) % 17 == 0
}

fn to_usize(n: &[&char]) -> usize {
    let mut sa = String::new();
    n.iter().for_each(|c| sa.push(**c));
    sa.parse().unwrap()
}

#[cfg(test)]
mod e43_tests {
    use crate::e43::{calc, is_substring_divisible};

    #[test]
    fn is_substring_divisible_works() {
        assert_eq!(true, is_substring_divisible(&vec! {&'1', &'4', &'0', &'6', &'3', &'5', &'7', &'2', &'8', &'9'}));
    }

    #[test]
    fn calc_works() {
        assert_eq!(16695334890, calc());
    }
}
