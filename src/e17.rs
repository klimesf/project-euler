pub(crate) fn e17() {
    println!("{}", calc(1000))
}

fn calc(n: usize) -> usize {
    (1..=n).map(|n| number_to_str(n)).map(|s| count_letters(s)).sum()
}

fn number_to_str(n: usize) -> String {
    if n <= 19 {
        return match n {
            0 => "".to_string(),
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            6 => "six".to_string(),
            7 => "seven".to_string(),
            8 => "eight".to_string(),
            9 => "nine".to_string(),
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            16 => "sixteen".to_string(),
            17 => "seventeen".to_string(),
            18 => "eighteen".to_string(),
            19 => "nineteen".to_string(),
            _ => {
                panic!()
            }
        };
    }

    if n <= 99 {
        return match n / 10 {
            2 => format!("twenty-{}", number_to_str(n % 10)),
            3 => format!("thirty-{}", number_to_str(n % 10)),
            4 => format!("forty-{}", number_to_str(n % 10)),
            5 => format!("fifty-{}", number_to_str(n % 10)),
            6 => format!("sixty-{}", number_to_str(n % 10)),
            7 => format!("seventy-{}", number_to_str(n % 10)),
            8 => format!("eighty-{}", number_to_str(n % 10)),
            9 => format!("ninety-{}", number_to_str(n % 10)),
            _ => {
                panic!()
            }
        };
    }

    if n <= 999 {
        let rem = n % 100;
        if rem == 0 {
            return match n / 100 {
                1 => "one hundred".to_string(),
                2 => "two hundred".to_string(),
                3 => "three hundred".to_string(),
                4 => "four hundred".to_string(),
                5 => "five hundred".to_string(),
                6 => "six hundred".to_string(),
                7 => "seven hundred".to_string(),
                8 => "eight hundred".to_string(),
                9 => "nine hundred".to_string(),
                _ => {
                    panic!()
                }
            };
        } else {
            return match n / 100 {
                1 => format!("one hundred and {}", number_to_str(n % 100)),
                2 => format!("two hundred and {}", number_to_str(n % 100)),
                3 => format!("three hundred and {}", number_to_str(n % 100)),
                4 => format!("four hundred and {}", number_to_str(n % 100)),
                5 => format!("five hundred and {}", number_to_str(n % 100)),
                6 => format!("six hundred and {}", number_to_str(n % 100)),
                7 => format!("seven hundred and {}", number_to_str(n % 100)),
                8 => format!("eight hundred and {}", number_to_str(n % 100)),
                9 => format!("nine hundred and {}", number_to_str(n % 100)),
                _ => {
                    panic!()
                }
            };
        }
    }

    if n == 1000 {
        return "one thousand".to_string();
    }

    panic!()
}

fn count_letters(s: String) -> usize {
    s.chars().filter(|c| !c.is_whitespace() && *c != '-').count()
}

#[cfg(test)]
mod e17_tests {
    use crate::e17::{calc, count_letters, number_to_str};

    #[test]
    fn count_letters_works() {
        assert_eq!(23, count_letters("three hundred and forty-two".to_string()));
        assert_eq!(20, count_letters("one hundred and fifteen".to_string()));
    }

    #[test]
    fn number_to_str_works() {
        assert_eq!("one".to_string(), number_to_str(1));
        assert_eq!("two".to_string(), number_to_str(2));
        assert_eq!("twenty-one".to_string(), number_to_str(21));
        assert_eq!("fifty-".to_string(), number_to_str(50));
        assert_eq!("ninety-nine".to_string(), number_to_str(99));
        assert_eq!("one hundred".to_string(), number_to_str(100));
        assert_eq!("three hundred and forty-two".to_string(), number_to_str(342));
        assert_eq!("one hundred and fifteen".to_string(), number_to_str(115));
        assert_eq!("nine hundred and ninety-nine".to_string(), number_to_str(999));
        assert_eq!("one thousand".to_string(), number_to_str(1000));
    }

    #[test]
    fn calc_works() {
        assert_eq!(19, calc(5));
    }
}
