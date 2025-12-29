use std::collections::HashMap;

pub(crate) fn e26() {
    println!("{}", longest_decimal_cycle())
}

fn longest_decimal_cycle() -> i32 {
    (1..1000)
        .max_by(|a, b| {
            let pa = period_len(*a);
            let pb = period_len(*b);
            pa.cmp(&pb)
        })
        .unwrap()
}

fn period_len(n: i32) -> i32 {
    let mut seen = HashMap::new();
    let mut remainder = 1;
    let mut period_len = 0;
    let mut i = 0;
    while remainder != 0 {
        if seen.contains_key(&remainder) {
            period_len = i - seen.get(&remainder).unwrap();
            break;
        }
        seen.insert(remainder, i);
        remainder = (remainder % n) * 10;
        i += 1;
    }
    period_len
}

#[cfg(test)]
mod e26_tests {
    use crate::e26::{longest_decimal_cycle, period_len};

    #[test]
    fn period_len_works() {
        assert_eq!(0, period_len(1)); // 1 (0)
        assert_eq!(0, period_len(2)); // 0.5 (0)
        assert_eq!(1, period_len(3)); // 0.3 (1)
        assert_eq!(0, period_len(4)); // 0.25 (0)
        assert_eq!(0, period_len(5)); // 0.2 (0)
        assert_eq!(1, period_len(6)); // 0.16 (1)
        assert_eq!(6, period_len(7)); // 0.142857 (6)
        assert_eq!(0, period_len(8)); // 0.125 (0)
        assert_eq!(1, period_len(9)); // 0.1 (1)
        assert_eq!(0, period_len(10)); // 0.1 (0)
    }

    #[test]
    fn longest_decimal_cycle_works() {
        assert_eq!(983, longest_decimal_cycle());
    }
}
