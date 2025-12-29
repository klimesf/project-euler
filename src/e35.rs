use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e35() {
    println!("{}", circular_primes(1000000))
}

fn circular_primes(below: usize) -> usize {
    let sieve = sieve_of_eratosthenes(below);
    let mut ans = 0;
    for i in 2..below {
        if !sieve[i] {
            continue;
        }

        let mut digs = to_digs(i);
        let mut all_prime = true;
        for _ in 0..=digs.len() {
            digs.rotate_left(1);
            let b = to_usize(&digs);
            if !sieve[b] {
                all_prime = false;
                break;
            }
        }

        if all_prime {
            ans += 1;
        }
    }
    ans
}

fn to_usize(n: &Vec<char>) -> usize {
    let mut sa = String::new();
    n.iter().for_each(|c| sa.push(*c));
    sa.parse().unwrap()
}

fn to_digs(mut n: usize) -> Vec<char> {
    let mut ans = vec![];
    while n > 0 {
        ans.push(char::from_digit((n % 10) as u32, 10).unwrap());
        n /= 10;
    }
    ans.reverse();
    ans
}

#[cfg(test)]
mod e35_tests {
    use crate::e35::circular_primes;

    #[test]
    fn circular_primes_works() {
        assert_eq!(13, circular_primes(100));
    }
}
