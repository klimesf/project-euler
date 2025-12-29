use num_integer::binomial;
use std::collections::VecDeque;

pub(crate) fn e15() {
    println!("{}", lattice_paths_combinatorial(20))
}

fn lattice_paths_combinatorial(n: usize) -> usize {
    binomial(2 * n, n)
}

#[allow(unused)]
fn lattice_paths_iterative(n: usize) -> usize {
    let mut ans = vec![vec! {0; n + 1}; n + 1];

    for c in 0..=n {
        ans[0][c] = 1;
    }
    for r in 0..=n {
        ans[r][0] = 1;
    }

    for r in 1..=n {
        for c in 1..=n {
            ans[r][c] = ans[r - 1][c] + ans[r][c - 1];
        }
    }

    ans[n][n]
}

#[allow(unused)]
fn lattice_paths_bfs(n: usize) -> usize {
    let mut ans = vec![vec! {0; n + 1}; n + 1];
    let mut stack = VecDeque::new();
    stack.push_back((0, 0));

    while let Some((r, c)) = stack.pop_front() {
        if r > n || c > n {
            continue;
        }
        ans[r][c] += 1;
        if (r, c) == (n, n) {
            continue;
        }

        stack.push_back((r + 1, c)); // down
        stack.push_back((r, c + 1)); // right
    }

    ans[n][n]
}

#[cfg(test)]
mod e15_tests {
    use crate::e15::{lattice_paths_bfs, lattice_paths_combinatorial, lattice_paths_iterative};

    #[test]
    fn lattice_paths_combinatorial_works() {
        assert_eq!(2, lattice_paths_combinatorial(1));
        assert_eq!(6, lattice_paths_combinatorial(2));
        assert_eq!(20, lattice_paths_combinatorial(3));
        assert_eq!(70, lattice_paths_combinatorial(4));
        assert_eq!(252, lattice_paths_combinatorial(5));
        assert_eq!(137846528820, lattice_paths_combinatorial(20));
    }

    #[test]
    fn lattice_paths_iterative_works() {
        assert_eq!(2, lattice_paths_iterative(1));
        assert_eq!(6, lattice_paths_iterative(2));
        assert_eq!(20, lattice_paths_iterative(3));
        assert_eq!(70, lattice_paths_iterative(4));
        assert_eq!(252, lattice_paths_iterative(5));
    }

    #[test]
    fn lattice_paths_bfs_works() {
        assert_eq!(2, lattice_paths_bfs(1));
        assert_eq!(6, lattice_paths_bfs(2));
        assert_eq!(20, lattice_paths_bfs(3));
        assert_eq!(70, lattice_paths_bfs(4));
        assert_eq!(252, lattice_paths_bfs(5));
    }
}
