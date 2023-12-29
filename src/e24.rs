use itertools::Itertools;

pub(crate) fn e24() {
    println!("{}", millionth_lexicographic_permutation())
}

fn millionth_lexicographic_permutation() -> i64 {
    let items = vec!{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
    let mut perms: Vec<Vec<&i64>> = items.iter().permutations(items.len()).collect();
    perms.sort();

    let mut ans = 0;
    for i in &perms[999999] {
        ans = ans * 10 + **i;
    }
    ans
}

#[cfg(test)]
mod e24_tests {
    use crate::e24::{millionth_lexicographic_permutation};

    #[test]
    fn millionth_lexicographic_permutation_works() {
        assert_eq!(0, millionth_lexicographic_permutation());
    }
}
