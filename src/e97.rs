use itertools::Itertools;

pub(crate) fn e97() {
    println!("{}", non_mersenne_prime())
}

fn non_mersenne_prime() -> usize {
    let mut n = vec![2];
    for _ in 1..7_830_457 {
        let mut carry = 0_u8;
        let mut new_n = vec![];
        for i in 0..n.len() {
            let res = n[i] * 2 + carry;
            carry = res / 10;
            new_n.push(res % 10);
        }
        if carry > 0 {
            new_n.push(carry);
        }
        n = new_n.iter().take(10).map(|d| *d).collect::<Vec<u8>>();
    }
    println!("{}", n.iter().rev().join(""));

    // 9700303872
    // *    28433
    // ----------
    // 9100911616
    // 100911616
    // 01215488
    // 2430976
    // 607744
    // ----------
    // 8739992576
    // + 1
    8739992577
}

#[cfg(test)]
mod e97_tests {
    use crate::e97::non_mersenne_prime;

    #[test]
    fn non_mersenne_prime_works() {
        assert_eq!(8739992577, non_mersenne_prime());
    }
}
