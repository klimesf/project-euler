use std::collections::HashMap;

pub(crate) fn e14() {
    println!("{}", longest_collatz(1000000))
}

fn longest_collatz(arg: usize) -> usize {
    let mut max = 0;
    let mut max_num = 13;
    let mut cache = HashMap::new();
    for i in 13..=arg {
        let len = collatz_len(i, &mut cache);
        if len > max {
            max = len;
            max_num = i;
        }
    }
    max_num
}

fn collatz_len(mut n: usize, cache: &mut HashMap<u32, u32>) -> usize {
    let mut len = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
            n = 3 * n + 1
        }
        len += 1
    }
    len + 1 // add 1 for the 1
}

#[cfg(test)]
mod e14_tests {
    use std::collections::HashMap;
    use crate::e14::{collatz_len, longest_collatz};

    #[test]
    fn e14_works() {
        assert_eq!(10, collatz_len(13, &mut HashMap::new()));
        assert_eq!(837799, longest_collatz(1000000));
    }
}
