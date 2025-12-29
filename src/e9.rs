pub(crate) fn e9() {
    println!("{}", calc(1000))
}

fn calc(sum: usize) -> usize {
    for a in 1..sum {
        for b in a..sum {
            for c in b..sum {
                if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == sum {
                    return a * b * c;
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod e9_tests {
    use crate::e9::calc;

    #[test]
    fn calc_works() {
        assert_eq!(3 * 4 * 5, calc(3 + 4 + 5));
    }
}
