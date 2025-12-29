pub(crate) fn e44() {
    println!("{}", calc())
}

fn calc() -> usize {
    let mut sieve = vec![false; 1000000000];
    let mut pentagonal = vec![];
    let mut n = 1;
    loop {
        let pentagonal_n = n * (3 * n - 1) / 2;
        if pentagonal_n >= sieve.len() {
            break;
        }
        sieve[pentagonal_n] = true;
        if n < 10000 {
            pentagonal.push(pentagonal_n);
        }
        n += 1;
    }

    let mut min = usize::MAX;
    for j in 0..pentagonal.len() - 1 {
        for k in j + 1..pentagonal.len() {
            let pj = pentagonal[j];
            let pk = pentagonal[k];
            if sieve[pj + pk] && sieve[pk - pj] && pk - pj < min {
                min = pk - pj;
            }
        }
    }
    min
}

#[cfg(test)]
mod e44_tests {
    use crate::e44::calc;

    #[test]
    fn calc_works() {
        assert_eq!(5482660, calc());
    }
}
