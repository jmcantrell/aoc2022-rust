use std::fmt;

use anyhow::{anyhow, Context};

use super::quinary::{Digit as UnbalancedDigit, Number as UnbalancedNumber, BASE};

use Digit::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Digit {
    Two,
    One,
    Zero,
    NegOne,
    NegTwo,
}

impl Digit {
    pub fn to_decimal(&self) -> isize {
        match self {
            Two => 2,
            One => 1,
            Zero => 0,
            NegOne => -1,
            NegTwo => -2,
        }
    }
}

impl Default for Digit {
    fn default() -> Self {
        Zero
    }
}

impl From<Digit> for char {
    fn from(digit: Digit) -> Self {
        match digit {
            Two => '2',
            One => '1',
            Zero => '0',
            NegOne => '-',
            NegTwo => '=',
        }
    }
}

macro_rules! impl_into_int {
    ($($t:ty),+) => {
        $(
            impl From<Digit> for $t {
                fn from(digit: Digit) -> $t {
                    match digit {
                        Two => 2,
                        One => 1,
                        Zero => 0,
                        NegOne => -1,
                        NegTwo => -2
                    }
                }
            }
        )+
    }
}

impl_into_int!(i8, i16, i32, i64, i128, isize);

impl TryFrom<char> for Digit {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Two),
            '1' => Ok(One),
            '0' => Ok(Zero),
            '-' => Ok(NegOne),
            '=' => Ok(NegTwo),
            _ => Err(anyhow!("invalid digit: {:?}", c)),
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number(Vec<Digit>);

impl Number {
    pub fn new(digits: Vec<Digit>) -> Self {
        Self(digits)
    }

    pub fn to_decimal(&self) -> isize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| (digit.to_decimal() * (BASE as usize).pow(i as u32) as isize))
            .sum()
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::new(vec![Digit::default()])
    }
}

impl TryFrom<&str> for Number {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let digits = s
            .chars()
            .enumerate()
            .map(|(i, c)| Digit::try_from(c).with_context(|| format!("character number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(digits))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.0.iter() {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

macro_rules! impl_from_int {
    ($($t:ty),+) => {
        $(
            impl From<$t> for Number {
                fn from(n: $t) -> Self {
                    let mut digits = Vec::from(UnbalancedNumber::from(n));
                    digits.reverse();
                    digits.push(UnbalancedDigit::Zero);

                    let mut balanced_digits = Vec::new();

                    for i in 0..digits.len() {
                        let digit = digits[i];
                        let (balanced_digit, carry) = digit.to_balanced();

                        balanced_digits.push(balanced_digit);

                        if !carry {
                            continue;
                        }

                        for digit in digits.iter_mut().skip(i + 1) {
                            let (new_digit, carry) = digit.carrying_add(UnbalancedDigit::One);
                            *digit = new_digit;
                            if !carry {
                                break;
                            }
                        }
                    }

                    while let Some(&Zero) = balanced_digits.last() {
                        balanced_digits.pop().unwrap();
                    }

                    balanced_digits.reverse();

                    Self(balanced_digits)
                }
            }
        )+
    };
}

impl_from_int!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_decimal() {
        macro_rules! assert_decimal {
            ($s:expr, $n:expr) => {
                assert_eq!(Number::try_from($s).unwrap().to_decimal(), $n);
            };
        }

        assert_decimal!("1", 1);
        assert_decimal!("2", 2);
        assert_decimal!("1=", 3);
        assert_decimal!("1-", 4);
        assert_decimal!("10", 5);
        assert_decimal!("11", 6);
        assert_decimal!("12", 7);
        assert_decimal!("2=", 8);
        assert_decimal!("2-", 9);
        assert_decimal!("20", 10);
        assert_decimal!("1=0", 15);
        assert_decimal!("1-0", 20);
        assert_decimal!("1=11-2", 2022);
        assert_decimal!("1-0---0", 12345);
        assert_decimal!("1121-1110-1=0", 314159265);
    }

    #[test]
    fn from_decimal() {
        macro_rules! assert_number {
            ($n:expr, $s:expr) => {
                assert_eq!(Number::try_from($n as usize).unwrap().to_string(), $s);
            };
        }

        assert_number!(1, "1");
        assert_number!(2, "2");
        assert_number!(3, "1=");
        assert_number!(4, "1-");
        assert_number!(5, "10");
        assert_number!(6, "11");
        assert_number!(7, "12");
        assert_number!(8, "2=");
        assert_number!(9, "2-");
        assert_number!(10, "20");
        assert_number!(15, "1=0");
        assert_number!(20, "1-0");
        assert_number!(2022, "1=11-2");
        assert_number!(12345, "1-0---0");
        assert_number!(314159265, "1121-1110-1=0");
    }
}
