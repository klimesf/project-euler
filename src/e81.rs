use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

pub(crate) fn e81() {
    println!("{}", path_sum(fs::read_to_string("input/e81/matrix.txt").unwrap()))
}

fn path_sum(input: String) -> usize {
    let matrix: Vec<Vec<usize>> = input
        .lines()
        .map(|line| line.split(",").map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();

    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(Pos { r: 0, c: 0, sum: 0 });

    let mut min = usize::MAX;
    while let Some(pos) = stack.pop() {
        let total = matrix[pos.r][pos.c] + pos.sum;
        if (pos.r, pos.c) == (matrix.len() - 1, matrix[0].len() - 1) {
            min = min.min(total);
            break;
        }
        if !visited.insert((pos.r, pos.c)) {
            continue;
        }

        if pos.r < matrix.len() - 1 {
            stack.push(Pos {
                r: pos.r + 1,
                c: pos.c,
                sum: total,
            });
        }
        if pos.c < matrix.len() - 1 {
            stack.push(Pos {
                r: pos.r,
                c: pos.c + 1,
                sum: total,
            });
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
mod e81_tests {
    use crate::e81::path_sum;
    use std::fs;

    #[test]
    fn path_sum_works() {
        assert_eq!(2427, path_sum(fs::read_to_string("input/e81/test.txt").unwrap()));
        assert_eq!(427337, path_sum(fs::read_to_string("input/e81/matrix.txt").unwrap()));
    }
}
