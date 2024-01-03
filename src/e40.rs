pub(crate) fn e40() {
    println!("{}", champernowne())
}

fn champernowne() -> u32 {
    let mut i = 0;
    let mut s = String::new();
    while s.len() <= 1000000 {
        s.push_str(format!("{}", i).as_str());
        i += 1;
    }
    let c: Vec<u32> = s.chars().map(|n| n.to_digit(10).unwrap()).collect();
    c[1] * c[10] * c[100] * c[1000] * c[10000] * c[100000] * c[1000000]
}

#[cfg(test)]
mod e40_tests {
    use crate::e40::{champernowne};

    #[test]
    fn champernowne_works() {
        assert_eq!(210, champernowne());
    }
}
