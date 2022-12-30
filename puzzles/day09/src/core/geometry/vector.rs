use anyhow::Context;

use std::fmt;

use super::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector {
    pub direction: Direction,
    pub magnitude: isize,
}

impl Vector {
    pub fn new(direction: Direction, magnitude: isize) -> Self {
        Self {
            direction,
            magnitude,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.magnitude)?;
        Ok(())
    }
}

impl TryFrom<&str> for Vector {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_int(s: &str) -> anyhow::Result<isize> {
            s.parse()
                .with_context(|| format!("invalid integer: {:?}", s))
        }

        let mut words = s.split_whitespace();

        let direction: Direction = words
            .next()
            .context("missing direction")?
            .chars()
            .next()
            .unwrap()
            .try_into()?;

        let magnitude: isize = parse_int(words.next().context("missing magnitude")?)?;

        Ok(Self {
            direction,
            magnitude,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        macro_rules! test {
            ($direction:expr, $magnitude:expr, $s:expr) => {
                let vector = Vector::new($direction, $magnitude);
                assert_eq!(vector.to_string(), $s);
                assert_eq!(Vector::try_from($s).unwrap(), vector);
            };
        }

        use Direction::*;

        test!(Up, 1, "U 1");
        test!(Down, 2, "D 2");
        test!(Left, 3, "L 3");
        test!(Right, 4, "R 4");
    }
}
