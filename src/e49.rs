use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e49() {
    println!("{}", calc())
}

fn calc() -> usize {
    let sieve = sieve_of_eratosthenes(9999);
    for i in 1000..=9999 {
        if !sieve[i] { continue }
        if i == 1487 { continue } // Skip the other one
        for j in i + 1..=9999 {
            if !sieve[j] { continue }
            for k in j + 1..=9999 {
                if !sieve[k] { continue }
                if j - i != k - j { continue }
                if is_permutation(i, j) && is_permutation(j, k) {
                    return i * 100000000 + j * 10000 + k;
                }
            }
        }
    }
    panic!("no luck")
}

fn is_permutation(mut a: usize, mut b: usize) -> bool {
    let mut digs_a = vec!();
    while a > 0 {
        digs_a.push(a % 10);
        a /= 10;
    }
    digs_a.sort();

    let mut digs_b = vec!();
    while b > 0 {
        digs_b.push(b % 10);
        b /= 10;
    }
    digs_b.sort();

    digs_a == digs_b
}

#[cfg(test)]
mod e49_tests {
    use crate::e49::{calc, is_permutation};

    #[test]
    fn is_permutation_works() {
        assert_eq!(true, is_permutation(1487, 4817));
        assert_eq!(false, is_permutation(11487, 4817));
    }

    #[test]
    fn calc_works() {
        assert_eq!(296962999629, calc());
    }
}
