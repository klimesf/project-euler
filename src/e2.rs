pub(crate) fn e2() {
    println!("{}", even_valued_fibb(4000000))
}

fn even_valued_fibb(below: usize) -> usize {
    let mut fibb = vec!{ 0; 100 };
    fibb[0] = 1;
    fibb[1] = 2;

    let mut ans = 2;
    let mut i = 2;
    loop {
        fibb[i] = fibb[i - 2] + fibb[i - 1];
        if fibb[i] > below { break }
        if fibb[i] % 2 == 0 { ans += fibb[i] }
        i += 1;
    }
    ans
}

#[cfg(test)]
mod e2_tests {
    use crate::e2::{even_valued_fibb};

    #[test]
    fn even_valued_fibb_works() {
        assert_eq!(44, even_valued_fibb(100));
    }
}
