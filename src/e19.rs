pub(crate) fn e19() {
    println!("{}", count_sunday_firsts())
}

fn count_sunday_firsts() -> usize {
    let mut date = (1901, 1, 1, 1); // 1 = tuesday
    let mut ans = 0;
    loop {
        if date.0 == 2000 && date.1 == 12 && date.2 == 31 { break; }

        let (y, m, d, dow) = date;

        if d == 1 && dow == 6 {
            ans += 1;
        }

        if d < 28 {
            date = (y, m, d + 1, (dow + 1) % 7);
            continue;
        }

        if d == 28 && m == 2 && y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
            date = (y, m, d + 1, (dow + 1) % 7);
            continue;
        }

        if d == 29 && m == 2 {
            date = (y, m + 1, 1, (dow + 1) % 7);
            continue;
        }

        if d < 30 {
            date = (y, m, d + 1, (dow + 1) % 7);
            continue;
        }

        if d == 30 && [4, 6, 9, 11].contains(&m) {
            date = (y, m + 1, 1, (dow + 1) % 7);
            continue;
        }

        if d < 31 {
            date = (y, m, d + 1, (dow + 1) % 7);
            continue;
        }

        if m == 12 {
            date = (y + 1, 1, 1, (dow + 1) % 7);
            continue;
        } else {
            date = (y, m + 1, 1, (dow + 1) % 7);
            continue;
        }
    }
    ans
}

#[cfg(test)]
mod e19_tests {
    use crate::e19::{count_sunday_firsts};

    #[test]
    fn count_sunday_firsts_works() {
        assert_eq!(171, count_sunday_firsts());
    }
}
