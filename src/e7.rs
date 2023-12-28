pub(crate) fn e7() {
    println!("{}", nth_prime(10001))
}

fn nth_prime(nth: usize) -> usize {
    if nth == 1 { return 2 }

    let mut prev_primes = vec!();
    let mut ctr = 2;
    let mut i = 3;
    loop {
        let mut prime = true;
        for prev in &prev_primes {
            if i % *prev == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            if ctr == nth { return i }
            ctr += 1;
            prev_primes.push(i);
        }
        i += 2;
    }
}

#[cfg(test)]
mod e7_tests {
    use crate::e7::{nth_prime};

    #[test]
    fn nth_prime_works() {
        assert_eq!(13, nth_prime(6));
    }
}
