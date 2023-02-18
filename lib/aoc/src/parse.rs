use super::Input;

pub trait Parse {
    type Parsed: std::fmt::Debug;

    fn new(input: Input) -> Self;

    fn parse(&self) -> anyhow::Result<Self::Parsed>;
}
