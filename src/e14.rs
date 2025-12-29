pub(crate) fn e14() {
    println!("{}", longest_collatz(1000000))
}

fn longest_collatz(arg: usize) -> usize {
    let mut max = 0;
    let mut max_num = 13;
    let mut cache = vec![0; 100000 * arg];
    for i in 13..=arg {
        let len = collatz_len_rec(i, &mut cache);
        if len > max {
            max = len;
            max_num = i;
        }
    }
    max_num
}

fn collatz_len_rec(n: usize, cache: &mut Vec<usize>) -> usize {
    if n == 1 {
        return 1;
    }
    if cache[n] != 0 {
        return cache[n];
    }

    let ans = if n % 2 == 0 {
        1 + collatz_len_rec(n / 2, cache)
    } else {
        1 + collatz_len_rec(3 * n + 1, cache)
    };
    cache[n] = ans;
    ans
}

#[cfg(test)]
mod e14_tests {
    use crate::e14::{collatz_len_rec, longest_collatz};

    #[test]
    fn e14_works() {
        assert_eq!(10, collatz_len_rec(13, &mut vec!(0; 100)));
        assert_eq!(837799, longest_collatz(1000000));
    }
}
