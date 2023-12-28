pub(crate) fn e1() {
    println!("{}", multiples_3_5(1000))
}

fn multiples_3_5(below: usize) -> usize {
    let mut ans = 0;
    for i in 0..below {
        if i % 3 == 0 || i % 5 == 0 {
            ans += i
        }
    }
    ans
}

#[cfg(test)]
mod e1_tests {
    use crate::e1::{multiples_3_5};

    #[test]
    fn multiples_3_5_works() {
        assert_eq!(23, multiples_3_5(10));
    }
}
