pub(crate) fn e55() {
    println!("{}", lyrchel_numbers())
}

fn lyrchel_numbers() -> usize {
    (5..10_000).filter(|&n| is_lyrchel_number(n)).count()
}

fn is_lyrchel_number(mut n: u128) -> bool {
    for _ in 0..50 {
        let rev_n = n.to_string().chars().rev().collect::<String>().parse::<u128>().unwrap();
        let res = n + rev_n;
        if is_palindrome(res) {
            return false;
        }
        n = res;
    }
    true
}

fn is_palindrome(n: u128) -> bool {
    n.to_string().chars().rev().collect::<String>() == n.to_string()
}

#[cfg(test)]
mod e55_tests {
    use crate::e55::lyrchel_numbers;

    #[test]
    fn lyrchel_numbers_works() {
        assert_eq!(249, lyrchel_numbers());
    }
}
