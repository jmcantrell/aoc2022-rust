use std::fmt;

use anyhow::{anyhow, Context};

use geometry::RelativeDirection;

use RelativeDirection::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    pub direction: RelativeDirection,
    pub magnitude: isize,
}

impl Movement {
    pub fn new(direction: RelativeDirection, magnitude: isize) -> Self {
        Self {
            direction,
            magnitude,
        }
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            match self.direction {
                Up => 'U',
                Down => 'D',
                Left => 'L',
                Right => 'R',
            },
            self.magnitude
        )?;

        Ok(())
    }
}

impl TryFrom<&str> for Movement {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_direction(s: &str) -> anyhow::Result<RelativeDirection> {
            match s {
                "U" => Ok(Up),
                "D" => Ok(Down),
                "L" => Ok(Left),
                "R" => Ok(Right),
                _ => Err(anyhow!("invalid direction: {:?}", s)),
            }
        }

        fn parse_int(s: &str) -> anyhow::Result<isize> {
            s.parse()
                .with_context(|| format!("invalid integer: {:?}", s))
        }

        let mut words = s.split_whitespace();

        let direction = parse_direction(words.next().context("missing direction")?)?;
        let magnitude = parse_int(words.next().context("missing magnitude")?)?;

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
                let movement = Movement::new($direction, $magnitude);
                assert_eq!(movement.to_string(), $s);
                assert_eq!(Movement::try_from($s).unwrap(), movement);
            };
        }

        test!(Up, 1, "U 1");
        test!(Down, 2, "D 2");
        test!(Left, 3, "L 3");
        test!(Right, 4, "R 4");
    }
}
