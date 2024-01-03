use std::collections::HashSet;
use itertools::Itertools;

pub(crate) fn e32() {
    println!("{}", pandigital_products())
}

fn pandigital_products() -> u32 {
    let digs = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut products = HashSet::new();

    let mut process = |va: Vec<char>, vb: Vec<char>| {
        let a = to_u32(&va);
        let b = to_u32(&vb);
        let mut rem: Vec<char> = digs.iter()
            .filter(|c| !va.contains(c) && !vb.contains(c))
            .map(|c| *c).collect();
        rem.sort();
        let mut prd = to_digs(a * b);
        prd.sort();

        if rem == prd {
            products.insert(a * b);
        }
    };

    digs.iter().permutations(5).for_each(|v| {
        // 12 x 345
        let va = v[0..2].iter().map(|c| **c).collect();
        let vb = v[2..5].iter().map(|c| **c).collect();
        process(va, vb);

        // 1 x 2345
        let va = v[0..1].iter().map(|c| **c).collect();
        let vb = v[1..5].iter().map(|c| **c).collect();
        process(va, vb);
    });

    products.iter().sum()
}

fn to_u32(n: &Vec<char>) -> u32 {
    let mut sa = String::new();
    n.iter().for_each(|c| sa.push(*c));
    sa.parse().unwrap()
}

fn to_digs(mut n: u32) -> Vec<char> {
    let mut ans = vec!();
    while n > 0 {
        ans.push(char::from_digit(n % 10, 10).unwrap());
        n /= 10;
    }
    ans
}

#[cfg(test)]
mod e32_tests {
    use crate::e32::{pandigital_products, to_digs};

    #[test]
    fn to_digs_works() {
        assert_eq!(vec! { '5', '4', '3', '2', '1' }, to_digs(12345));
    }

    #[test]
    fn pandigital_products_works() {
        assert_eq!(45228, pandigital_products());
    }
}
