use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e50() {
    println!("{}", consecutive_prime_sum(1000000))
}

fn consecutive_prime_sum(below: usize) -> usize {
    let sieve = sieve_of_eratosthenes(below);

    let mut max= 0;
    let mut ans = 0;

    for i in 2..sieve.len() {
        if !sieve[i] { continue }
        let mut sum = i;
        let mut max_prime = i;
        let mut max_consecutive = 1;
        let mut j = i + 1;
        let mut consecutive = 1;
        while sum < below && j < sieve.len() {
            if !sieve[j] {
                j += 1;
                continue
            }
            sum += j;
            if sum >= below { break }
            if sieve[sum] {
                max_prime = sum;
                max_consecutive = consecutive;
            }
            j += 1;
            consecutive += 1;
        }

        if max_consecutive > max {
            max = max_consecutive;
            ans = max_prime;
        }
    }

    ans
}

#[cfg(test)]
mod e50_tests {
    use crate::e50::{consecutive_prime_sum};

    #[test]
    fn consecutive_prime_sum_works() {
        assert_eq!(41, consecutive_prime_sum(100));
        assert_eq!(953, consecutive_prime_sum(1000));
        assert_eq!(997651, consecutive_prime_sum(1000000));
    }
}
