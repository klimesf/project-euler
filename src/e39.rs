pub(crate) fn e39() {
    println!("{}", integer_right_triangles(1000).0)
}

fn integer_right_triangles(below: usize) -> (usize, usize) {
    let mut max = 0;
    let mut max_p = 0;
    for p in 1..=below {
        let pc = count_solutions(p);
        if pc > max {
            max = pc;
            max_p = p;
        }
    }
    (max_p, max)
}

fn count_solutions(p: usize) -> usize {
    let mut ans = 0;
    for a in 1..p / 3 {
        for b in a..(p - a) / 2 {
            let c = p - a - b;
            if a + b + c == p && a.pow(2) + b.pow(2) == c.pow(2) {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod e39_tests {
    use crate::e39::{integer_right_triangles, count_solutions};

    #[test]
    fn count_solutions_works() {
        assert_eq!(3, count_solutions(120));
    }

    #[test]
    fn integer_right_triangles_works() {
        assert_eq!((840, 8), integer_right_triangles(1000));
    }
}
