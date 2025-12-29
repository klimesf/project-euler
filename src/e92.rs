use memoize::memoize;

pub(crate) fn e92() {
    println!("{}", calc())
}

fn calc() -> usize {
    let mut ans = 0;
    for n in 1..10000000 {
        let res = chain(n);
        if res == 89 {
            ans += 1
        }
    }
    ans
}

#[memoize]
fn chain(n: usize) -> usize {
    if n == 1 || n == 89 {
        return n;
    }

    let mut new_n = 0;
    let mut rem = n;
    while rem > 0 {
        new_n += (rem % 10) * (rem % 10);
        rem /= 10;
    }

    chain(new_n)
}

#[cfg(test)]
mod e92_tests {
    use crate::e92::calc;

    #[test]
    fn calc_works() {
        assert_eq!(8581146, calc());
    }
}
