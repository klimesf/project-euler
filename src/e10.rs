pub(crate) fn e10() {
    println!("{}", calc(2000000))
}

fn calc(n: usize) -> usize {
    let mut sieve = vec!{ true; n + 1 };

    let mut ans = 0;
    for i in 2..=(n as f64).sqrt() as usize {
        if sieve[i] {
            ans += i;
            for j in (i.pow(2)..=n).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    for i in (n as f64).sqrt() as usize + 1..=n {
        if sieve[i] {
            ans += i;
        }
    }
    ans
}

#[cfg(test)]
mod e10_tests {
    use crate::e10::{calc};

    #[test]
    fn calc_works() {
        assert_eq!(17, calc(10));
    }
}
