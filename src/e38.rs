use crate::utils::toolbox::is_pandigital;

pub(crate) fn e38() {
    println!("{}", pandigital_multiples())
}

fn pandigital_multiples() -> usize {
    let mut max = 123456789;

    for i in 1..100000 {
        let mut sum = i;
        for n in 2..10 {
            if concatenate(sum, i * n) > 987654321 {
                break;
            }
            sum = concatenate(sum, i * n);
        }
        if i != sum && is_pandigital(sum) && sum > max {
            max = sum;
        }
    }

    max
}

fn concatenate(a: usize, b: usize) -> usize {
    let mut times = 1;
    let mut b_rem = b;
    while b_rem > 0 {
        b_rem /= 10;
        times *= 10;
    }

    a * times + b
}

#[cfg(test)]
mod e38_tests {
    use crate::e38::{concatenate, pandigital_multiples};

    #[test]
    fn concatenate_works() {
        assert_eq!(192384, concatenate(192, 384));
        assert_eq!(123456, concatenate(1234, 56));
        assert_eq!(12, concatenate(1, 2));
        assert_eq!(99198, concatenate(99, 198));
    }

    #[test]
    fn pandigital_multiples_works() {
        assert_eq!(932718654, pandigital_multiples());
    }
}
