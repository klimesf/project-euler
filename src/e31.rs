pub(crate) fn e31() {
    println!("{}", coin_sums())
}

fn coin_sums() -> usize {
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];

    let mut ans = 0;
    for a in 0..=200 {
        for b in 0..=100 {
            for c in 0..=40 {
                for d in 0..=20 {
                    for e in 0..=10 {
                        for f in 0..=4 {
                            for g in 0..=2 {
                                for h in 0..=1 {
                                    if (coins[0] * a + coins[1] * b + coins[2] * c + coins[3] * d
                                        + coins[4] * e + coins[5] * f + coins[6] * g + coins[7] * h) == 200 {
                                        ans += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod e31_tests {
    use crate::e31::{coin_sums};

    #[test]
    fn coin_sums_works() {
        assert_eq!(73682, coin_sums());
    }
}
