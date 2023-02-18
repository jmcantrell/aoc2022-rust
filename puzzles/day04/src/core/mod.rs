pub type Section = usize;
pub type SectionRange = std::ops::RangeInclusive<Section>;

pub mod elf;
pub use elf::*;
