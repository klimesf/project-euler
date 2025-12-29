use std::fs;

pub(crate) fn e22() {
    println!("{}", names_scores(fs::read_to_string("input/e22/input.txt").unwrap()))
}

fn names_scores(input: String) -> usize {
    let mut names: Vec<&str> = input.lines().collect();
    names.sort();
    names
        .iter()
        .enumerate()
        .map(|(i, name)| (i + 1) * name_score(name))
        .sum()
}

fn name_score(s: &str) -> usize {
    s.chars().map(|c| (c as u8 - b'A' + 1) as usize).sum()
}

#[cfg(test)]
mod e22_tests {
    use crate::e22::{name_score, names_scores};
    use std::fs;

    #[test]
    fn name_score_works() {
        assert_eq!(53, name_score("COLIN"));
    }

    #[test]
    fn names_scores_works() {
        assert_eq!(0, names_scores(fs::read_to_string("input/e22/input.txt").unwrap()));
    }
}
