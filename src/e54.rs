use std::cmp::Ordering;
use std::fs;

pub(crate) fn e54() {
    println!("{}", poker_hands(fs::read_to_string("input/e54/poker.txt").unwrap()))
}

fn poker_hands(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let cards: Vec<&str> = line.split_whitespace().collect();
            let player1 = Hand::new(cards[0..5].iter().map(|s| Card::new(s)).collect());
            let player2 = Hand::new(cards[5..].iter().map(|s| Card::new(s)).collect());
            println!("{:?} {:?}", player1, player2);

            match player1.cmp(&player2) {
                Ordering::Less => 0,
                Ordering::Equal => {
                    panic!("hands are equal")
                }
                Ordering::Greater => 1,
            }
        })
        .sum()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}

impl Ord for Hand {
    fn cmp(&self, _other: &Self) -> Ordering {
        todo!()
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Card {
    value: usize,
    color: char,
}

impl Card {
    pub fn new(raw: &&str) -> Self {
        let raw_value = raw.chars().nth(0).unwrap();
        let color = raw.chars().nth(1).unwrap();
        let value = match raw_value {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 7,
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => {
                panic!("unknown card {}", raw_value)
            }
        };
        Self { value, color }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod e54_tests {
    use crate::e54::poker_hands;
    use std::fs;

    #[test]
    fn calc_works() {
        assert_eq!(0, poker_hands(fs::read_to_string("input/e54/poker.txt").unwrap()));
    }
}
