pub type Name<'a> = &'a str;
pub type Value = isize;

pub mod operation;
pub use operation::*;

pub mod expression;
pub use expression::*;

pub mod job;
pub use job::*;

pub mod monkey_troop;
pub use monkey_troop::*;
