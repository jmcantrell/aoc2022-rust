pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        macro_rules! assert_gcd {
            ($a:expr, $b:expr, $expected:expr) => {
                assert_eq!(gcd($a, $b), $expected);
                assert_eq!(gcd($b, $a), $expected);
            };
        }

        assert_gcd!(1, 2, 1);
        assert_gcd!(20, 2, 2);
        assert_gcd!(20, 10, 10);
        assert_gcd!(25, 35, 5);
    }
}
