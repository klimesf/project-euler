use std::fs;

pub(crate) fn e18() {
    println!("{}", max_path_sum(fs::read_to_string("input/e18/input.txt").unwrap()))
}

fn max_path_sum(input: String) -> usize {
    let triangle: Vec<Vec<usize>> = input
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse::<usize>().unwrap()).collect())
        .collect();

    let mut stack = vec![];
    stack.push((0, 0, 0));

    let mut max = 0;
    while let Some((r, c, total)) = stack.pop() {
        if r == triangle.len() {
            max = max.max(total);
            continue;
        }

        // Pruning
        let potential_best = (triangle.len() - r) * 99 + total;
        if potential_best < max {
            continue;
        }

        let curr = triangle[r][c];
        stack.push((r + 1, c, total + curr));
        stack.push((r + 1, c + 1, total + curr));
    }
    max
}

#[cfg(test)]
mod e18_tests {
    use crate::e18::max_path_sum;
    use std::fs;

    #[test]
    fn max_path_sum_works() {
        assert_eq!(23, max_path_sum(fs::read_to_string("input/e18/test.txt").unwrap()));
    }
}
