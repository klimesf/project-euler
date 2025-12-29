pub(crate) fn e4() {
    println!("{}", largest_palindrome(3))
}

fn largest_palindrome(n: usize) -> usize {
    let mut range = 9;
    for _ in 1..n {
        range = range * 10 + 9
    }
    let mut max = 0;
    for i in (1..=range).rev() {
        for j in (i..=range).rev() {
            if is_palindrome(i * j) {
                max = max.max(i * j)
            }
        }
    }
    max
}

fn is_palindrome(n: usize) -> bool {
    n == reverse(n)
}

fn reverse(n: usize) -> usize {
    let mut tmp = n;
    let mut reversed = 0;
    while tmp > 0 {
        reversed = reversed * 10 + tmp % 10;
        tmp /= 10;
    }
    return reversed;
}

#[cfg(test)]
mod e4_tests {
    use crate::e4::{is_palindrome, largest_palindrome};

    #[test]
    fn is_palindrome_works() {
        assert_eq!(true, is_palindrome(9009));
        assert_eq!(false, is_palindrome(9008));
        assert_eq!(true, is_palindrome(906609));
        assert_eq!(false, is_palindrome(906608));
    }

    #[test]
    fn largest_prime_factor_works() {
        assert_eq!(9009, largest_palindrome(2));
    }
}
