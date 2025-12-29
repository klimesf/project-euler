pub(crate) fn e28() {
    println!("{}", spiral_diagonals(1001))
}

fn spiral_diagonals(dim: usize) -> usize {
    let mut ans = 0;
    let mut dir = (0, 1, 1);
    let mut pos = (0, 0);
    let mut i = 1;
    'outer: loop {
        for _ in 0..dir.2 {
            if i > dim * dim {
                break 'outer;
            }
            if pos.0 == pos.1 || pos.0 == -pos.1 {
                ans += i;
            }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            i += 1;
        }

        dir = match (dir.0, dir.1) {
            (0, 1) => (1, 0, dir.2),
            (1, 0) => (0, -1, dir.2 + 1),
            (0, -1) => (-1, 0, dir.2),
            (-1, 0) => (0, 1, dir.2 + 1),
            (_, _) => {
                panic!()
            }
        };
    }
    ans
}

#[cfg(test)]
mod e28_tests {
    use crate::e28::spiral_diagonals;

    #[test]
    fn spiral_diagonals_works() {
        assert_eq!(101, spiral_diagonals(5));
    }
}
