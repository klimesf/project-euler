#[allow(unused)]
pub(crate) fn e719() {
    println!("{}", number_splitting())
}

#[allow(unused)]
fn number_splitting() -> u64 {
    // 1*1 = 1, 2*2 = 4, 3*3 = 9 => those cannot be split into 2 or more numbers
    // 4*4 up to 8*8 are not S-numbers
    //
    // sqrt(1_000_000_000_000) == 1_000_000, that's the max possible base
    (9..=1_000_000).filter(|&n| is_s_number(n)).map(|n| n * n).sum()
}

fn is_s_number(n: u64) -> bool {
    let pow = n * n;
    let _digits = pow
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    todo!()
}

#[cfg(test)]
mod e719_tests {
    use crate::e719::number_splitting;

    #[test]
    fn number_splitting_works() {
        assert_eq!(0, number_splitting());
    }
}
