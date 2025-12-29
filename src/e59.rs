use itertools::Itertools;
use std::fs;

pub(crate) fn e59() {
    println!("{}", xor_decryption(fs::read_to_string("input/e59/input.txt").unwrap()))
}

fn xor_decryption(input: String) -> usize {
    let cipher = input.split(',').map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>();
    let possible_chars = (b'a'..=b'z').collect::<Vec<_>>();
    possible_chars
        .iter()
        .permutations(3)
        .map(|perm| {
            cipher
                .iter()
                .enumerate()
                .map(|(i, &c)| c ^ perm[i % perm.len()])
                .map(|c| c as char)
                .join("")
        })
        .filter(|s| s.contains("elegant"))
        .last()
        .unwrap()
        .chars()
        .map(|c| c as usize)
        .sum()
}

#[cfg(test)]
mod e59_tests {
    use crate::e59::xor_decryption;
    use std::fs;

    #[test]
    fn xor_decryption_works() {
        assert_eq!(129448, xor_decryption(fs::read_to_string("input/e59/input.txt").unwrap()));
    }
}
