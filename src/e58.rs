use crate::utils::toolbox::sieve_of_eratosthenes;

pub(crate) fn e58() {
    println!("{}", spiral_primes())
}

fn spiral_primes() -> usize {
    let sieve = sieve_of_eratosthenes(1000000000);
    let mut dir = (0, 1, 1);
    let mut pos = (0, 0);
    let mut i = 1;
    let mut dim = 1;
    let mut primes = 0;
    let mut totals = 0;
    'outer: loop { // The loop is reused from e28 and it is clockwise, but it doesn't matter
        for _ in 0..dir.2 {
            if i > dim * dim {
                if dim > 1 && 10 * primes < totals {
                    break 'outer;
                }
                dim += 1;
            }
            if pos.0 == pos.1 || pos.0 == -pos.1 {
                totals += 1;
                if sieve[i] { primes += 1 }
            }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            i += 1;
        }

        dir = match (dir.0, dir.1) {
            (0, 1) => { (1, 0, dir.2) }
            (1, 0) => { (0, -1, dir.2 + 1) }
            (0, -1) => { (-1, 0, dir.2) }
            (-1, 0) => { (0, 1, dir.2 + 1) }
            (_, _) => { panic!() }
        };
    }
    dim
}

#[cfg(test)]
mod e58_tests {
    use crate::e58::{spiral_primes};

    #[test]
    fn spiral_primes_works() {
        assert_eq!(26241, spiral_primes());
    }
}
