use std::convert::TryFrom;

use anyhow::Context;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn ensure_prefix<'a>(s: &'a str, prefix: &'a str) -> anyhow::Result<&'a str> {
            s.trim()
                .strip_prefix(prefix)
                .with_context(|| format!("expected string to start with: {:?}", prefix))
        }

        fn parse_int(s: &str) -> anyhow::Result<isize> {
            s.trim()
                .parse()
                .with_context(|| format!("invalid integer: {:?}", s))
        }

        let mut words = s.split(",");

        let x = parse_int(ensure_prefix(
            words.next().context("missing x component")?,
            "x=",
        )?)?;

        let y = parse_int(ensure_prefix(
            words.next().context("missing y component")?,
            "y=",
        )?)?;

        Ok(Self { x, y })
    }
}
