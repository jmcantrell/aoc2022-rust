use std::fmt;

use anyhow::{anyhow, Context};

use super::balanced_quinary::Digit as BalancedDigit;

use Digit::*;

pub const BASE: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl Digit {
    pub fn to_decimal(&self) -> u8 {
        match self {
            Zero => 0,
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
        }
    }

    pub fn carrying_add(self, other: Self) -> (Self, bool) {
        let result = self.to_decimal() + other.to_decimal();
        ((result % BASE).try_into().unwrap(), result / BASE != 0)
    }

    pub fn to_balanced(self) -> (BalancedDigit, bool) {
        match self {
            Zero => (BalancedDigit::Zero, false),
            One => (BalancedDigit::One, false),
            Two => (BalancedDigit::Two, false),
            Three => (BalancedDigit::NegTwo, true),
            Four => (BalancedDigit::NegOne, true),
        }
    }
}

impl Default for Digit {
    fn default() -> Self {
        Zero
    }
}

macro_rules! impl_try_from_int {
    ($($t:ty),+) => {
        $(
            impl TryFrom<$t> for Digit {
                type Error = anyhow::Error;

                fn try_from(n: $t) -> Result<Self, Self::Error> {
                    match n {
                        0 => Ok(Zero),
                        1 => Ok(One),
                        2 => Ok(Two),
                        3 => Ok(Three),
                        4 => Ok(Four),
                        _ => Err(anyhow!("invalid digit: {:?}", n)),
                    }
                }
            }
        )+
    };
}

impl_try_from_int!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

impl From<Digit> for char {
    fn from(digit: Digit) -> Self {
        match digit {
            Zero => '0',
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
        }
    }
}

impl TryFrom<char> for Digit {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
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

    pub fn to_decimal(&self) -> usize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| (digit.to_decimal() * BASE.pow(i as u32)) as usize)
            .sum()
    }
}

impl From<Number> for Vec<Digit> {
    fn from(number: Number) -> Self {
        number.0
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::new(vec![Digit::default()])
    }
}

macro_rules! impl_from_uint {
    ($($t:ty),+) => {
        $(
            impl From<$t> for Number {
                fn from(mut n: $t) -> Self {
                    let mut digits = Vec::new();

                    let base = BASE as $t;

                    while n != 0 {
                        let q = n / base;
                        let r = n % base;
                        digits.push(r.try_into().unwrap());
                        n = q;
                    }

                    if digits.is_empty() {
                        digits.push(Zero);
                    } else {
                        digits.reverse();
                    }

                    Self(digits)
                }
            }
        )+
    };
}

impl_from_uint!(u8, u16, u32, u64, u128, usize);

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

        assert_decimal!("0", 0);
        assert_decimal!("1", 1);
        assert_decimal!("2", 2);
        assert_decimal!("3", 3);
        assert_decimal!("4", 4);
        assert_decimal!("10", 5);
        assert_decimal!("11", 6);
        assert_decimal!("123", 38);
    }

    #[test]
    fn from_decimal() {
        macro_rules! assert_number {
            ($n:expr, $s:expr) => {
                assert_eq!(Number::try_from($n as usize).unwrap().to_string(), $s);
            };
        }

        assert_number!(0, "0");
        assert_number!(1, "1");
        assert_number!(2, "2");
        assert_number!(3, "3");
        assert_number!(4, "4");
        assert_number!(5, "10");
        assert_number!(6, "11");
        assert_number!(38, "123");
    }

    #[test]
    fn carrying_add() {
        assert_eq!(Zero.carrying_add(One), (One, false));
        assert_eq!(One.carrying_add(One), (Two, false));
        assert_eq!(Two.carrying_add(One), (Three, false));
        assert_eq!(Three.carrying_add(One), (Four, false));
        assert_eq!(Four.carrying_add(One), (Zero, true));
    }
}
