use std::fs;

pub(crate) fn e11() {
    println!("{}", calc(fs::read_to_string("input/e11/input.txt").unwrap()))
}

fn calc(input: String) -> u64 {
    let matrix: Vec<Vec<u64>> = input.lines().map(|line| {
        line.split(" ").map(|s| s.parse().unwrap()).collect()
    }).collect();

    let mut max = 0;

    // Horizontally
    for r in 0..matrix.len() {
        for c in 3..matrix[r].len() {
            let prd = matrix[r][c - 3] * matrix[r][c - 2] * matrix[r][c - 1] * matrix[r][c];
            max = max.max(prd);
        }
    }

    // Vertically
    for r in 3..matrix.len() {
        for c in 0..matrix[r].len() {
            let prd = matrix[r - 3][c] * matrix[r - 2][c] * matrix[r - 1][c] * matrix[r][c];
            max = max.max(prd);
        }
    }

    // Diagonally \
    for dr in 0..matrix.len() - 3 {
        for dc in 0..matrix.len() - 3 {
            for r in 3 + dr..matrix.len() {
                for c in 3 + dc..matrix[r].len() {
                    let prd = matrix[r - 3][c - 3] * matrix[r - 2][c - 2] * matrix[r - 1][c - 1] * matrix[r][c];
                    max = max.max(prd);
                }
            }
        }
    }

    // Diagonally /
    for dr in 0..matrix.len() - 3 {
        for dc in 0..matrix.len() - 3 {
            for r in 3 + dr..matrix.len() {
                for c in 3 + dc..matrix[r].len() {
                    let prd = matrix[r][c - 3] * matrix[r - 1][c - 2] * matrix[r - 2][c - 1] * matrix[r - 3][c];
                    max = max.max(prd);
                }
            }
        }
    }

    max
}

#[cfg(test)]
mod e11_tests {
    use std::fs;
    use crate::e11::{calc};

    #[test]
    fn calc_works() {
        assert_eq!(70600674, calc(fs::read_to_string("input/e11/input.txt").unwrap()));
    }
}
