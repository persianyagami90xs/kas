//! Utility data types

/// An `(x, y)` coordinate or a size.
pub type Coord = (i32, i32);

/// A rectangular region.
#[derive(Clone, Default, Debug)]
pub struct Rect {
    pub pos: Coord,
    pub size: Coord,
}