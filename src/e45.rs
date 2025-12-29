pub(crate) fn e45() {
    println!("{}", calc())
}

fn calc() -> usize {
    let mut sieve_triangle = vec![false; 10000000000];
    let mut sieve_pentagonal = vec![false; 10000000000];
    let mut sieve_hexagonal = vec![false; 10000000000];
    let mut n = 1;
    loop {
        let triangle_n = n * (n + 1) / 2;
        if triangle_n < sieve_triangle.len() {
            sieve_triangle[triangle_n] = true
        }
        let pentagonal_n = n * (3 * n - 1) / 2;
        if pentagonal_n < sieve_pentagonal.len() {
            sieve_pentagonal[pentagonal_n] = true
        }
        let hexagonal_n = n * (2 * n - 1);
        if hexagonal_n < sieve_hexagonal.len() {
            sieve_hexagonal[hexagonal_n] = true
        }

        if triangle_n >= sieve_triangle.len()
            && pentagonal_n >= sieve_pentagonal.len()
            && hexagonal_n >= sieve_hexagonal.len()
        {
            break;
        }

        n += 1;
    }

    for i in 40755 + 1..sieve_triangle.len() {
        if sieve_triangle[i] && sieve_pentagonal[i] && sieve_hexagonal[i] {
            return i;
        }
    }
    panic!("no luck")
}

#[cfg(test)]
mod e45_tests {
    use crate::e45::calc;

    #[test]
    fn calc_works() {
        assert_eq!(1533776805, calc());
    }
}
