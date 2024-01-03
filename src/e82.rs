use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

pub(crate) fn e82() {
    println!("{}", path_sum(fs::read_to_string("input/e81/matrix.txt").unwrap()))
}

fn path_sum(input: String) -> usize {
    let matrix: Vec<Vec<usize>> = input.lines().map(|line| {
        line.split(",").map(|n| n.parse::<usize>().unwrap()).collect()
    }).collect();

    let mut min = usize::MAX;
    for sr in 0..matrix.len() {
        let mut stack = BinaryHeap::new();
        let mut visited = HashSet::new();
        stack.push(Pos { r: sr, c: 0, sum: 0 });
        while let Some(pos) = stack.pop() {
            let total = matrix[pos.r][pos.c] + pos.sum;
            if pos.c == matrix[0].len() - 1 {
                min = min.min(total);
                break;
            }
            if !visited.insert((pos.r, pos.c)) {
                continue;
            }

            if pos.r > 0 {
                stack.push(Pos { r: pos.r - 1, c: pos.c, sum: total });
            }
            if pos.r < matrix.len() - 1 {
                stack.push(Pos { r: pos.r + 1, c: pos.c, sum: total });
            }
            if pos.c < matrix.len() - 1 {
                stack.push(Pos { r: pos.r, c: pos.c + 1, sum: total });
            }
        }
    }
    min
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Pos {
    r: usize,
    c: usize,
    sum: usize,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sum.cmp(&self.sum)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod e82_tests {
    use std::fs;
    use crate::e82::{path_sum};

    #[test]
    fn path_sum_works() {
        assert_eq!(994, path_sum(fs::read_to_string("input/e81/test.txt").unwrap()));
        assert_eq!(260324, path_sum(fs::read_to_string("input/e81/matrix.txt").unwrap()));
    }
}
