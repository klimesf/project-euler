use std::fs;

pub(crate) fn e67() {
    println!("{}", max_path_sum(fs::read_to_string("input/e67/input.txt").unwrap()))
}

fn max_path_sum(input: String) -> usize {
    let mut triangle: Vec<Vec<usize>> = input.lines().map(|line| {
        line.split(" ").map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect();

    // Go from the bottom of the triangle
    // For each pair, add the max to the element above (the one where the path would choose between a and b)
    // The max path will be on the top of the triangle once we are finished
    for r in (0..triangle.len()).rev() {
        for c in (1..triangle[r].len()).rev() {
            let a = triangle[r][c];
            let b = triangle[r][c - 1];

            triangle[r - 1][c - 1] = triangle[r - 1][c - 1] + a.max(b);
        }
    }

    triangle[0][0]
}

#[cfg(test)]
mod e67_tests {
    use std::fs;
    use crate::e67::{max_path_sum};

    #[test]
    fn max_path_sum_works() {
        assert_eq!(23, max_path_sum(fs::read_to_string("input/e67/test.txt").unwrap()));
    }
}
