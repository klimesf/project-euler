use std::fs;

pub(crate) fn e42() {
    println!("{}", calc(fs::read_to_string("input/e42/words.txt").unwrap()))
}

fn calc(input: String) -> usize {
    let mut sieve = vec![false; 100000];
    let mut n = 1;
    loop {
        let triangle_n = n * (n + 1) / 2;
        if triangle_n >= sieve.len() {
            break;
        }
        sieve[triangle_n] = true;
        n += 1;
    }

    input
        .lines()
        .map(|line| alphabetical_position(line))
        .filter(|ap| sieve[*ap])
        .count()
}

fn alphabetical_position(word: &str) -> usize {
    let cs: Vec<char> = word.chars().collect();
    let mut sum = 0;
    for c in cs {
        sum += (c as u8 - b'A' + 1) as usize;
    }
    sum
}

#[cfg(test)]
mod e42_tests {
    use crate::e42::{alphabetical_position, calc};
    use std::fs;

    #[test]
    fn alphabetical_position_works() {
        assert_eq!(55, alphabetical_position("SKY"));
    }

    #[test]
    fn calc_works() {
        assert_eq!(162, calc(fs::read_to_string("input/e42/words.txt").unwrap()));
    }
}
