pub(crate) fn e53() {
    println!("{}", combinatoric_selections())
}

fn combinatoric_selections() -> usize {
    // Build Pascal's triangle, values capped at 1_000_000
    // Note: The triangle is on a hex grid
    //   0,0   0,1   0,2
    //     1,0    1,1   1,2
    //   2,0   2,1   2,2
    let mut ans = 0;
    let mut triangle = vec![vec![0; 101]; 101];
    triangle[0][50] = 1;
    for i in 1..=100 {
        for j in 1..100 {
            triangle[i][j] = if i % 2 == 0 {
                triangle[i - 1][j - 1] + triangle[i - 1][j]
            } else {
                triangle[i - 1][j] + triangle[i - 1][j + 1]
            };
            if triangle[i][j] > 1_000_000 {
                ans += 1;
                triangle[i][j] = 1_000_001; // To not overflow the int
            }
        }
    }
    ans
}

#[cfg(test)]
mod e53_tests {
    use crate::e53::combinatoric_selections;

    #[test]
    fn combinatoric_selections_works() {
        assert_eq!(4075, combinatoric_selections());
    }
}
