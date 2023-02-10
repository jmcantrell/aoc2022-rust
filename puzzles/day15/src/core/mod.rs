pub type Point = geometry::Location<isize>;
pub type Extents = geometry::Rectangle<isize>;

pub mod range;
pub use range::*;

pub mod scan;
pub use scan::*;

pub mod taxicab;
pub use taxicab::*;
