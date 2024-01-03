pub(crate) fn e36() {
    println!("{}", double_base_palindromes(1000000))
}

fn double_base_palindromes(below: usize) -> usize {
    let mut ans = 0;
    for i in (1..below).step_by(2) {
        if is_palindrome(format!("{}", i)) && is_palindrome(format!("{:b}", i)) {
            ans += i;
        }
    }
    ans
}

fn is_palindrome(n: String) -> bool {
    let cs: Vec<char> = n.chars().collect();
    for i in 0..cs.len() / 2 {
        if cs[i] != cs[cs.len() - i - 1] { return false }
    }
    true
}

#[cfg(test)]
mod e36_tests {
    use crate::e36::{double_base_palindromes, is_palindrome};

    #[test]
    fn is_palindrome_works() {
        assert_eq!(true, is_palindrome("585".to_string()));
        assert_eq!(false, is_palindrome("586".to_string()));
        assert_eq!(true, is_palindrome("1001001001".to_string()));
    }

    #[test]
    fn double_base_palindromes_works() {
        assert_eq!(872187, double_base_palindromes(1000000));
    }
}
