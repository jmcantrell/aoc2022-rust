use super::Direction;

use Direction::*;

pub type Coord = usize;
pub type LocationTuple = (Coord, Coord);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Location(LocationTuple);

impl Location {
    pub fn new(row: Coord, column: Coord) -> Self {
        Self((row, column))
    }

    pub fn inner(&self) -> &LocationTuple {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut LocationTuple {
        &mut self.0
    }

    pub fn into_inner(self) -> LocationTuple {
        self.into()
    }

    pub fn row(&self) -> &Coord {
        &self.inner().0
    }

    pub fn row_mut(&mut self) -> &mut Coord {
        &mut self.inner_mut().0
    }

    pub fn column(&self) -> &Coord {
        &self.inner().1
    }

    pub fn column_mut(&mut self) -> &mut Coord {
        &mut self.inner_mut().1
    }

    pub fn neighbor(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Up => self
                .row()
                .checked_sub(1)
                .map(|row| Location::new(row, *self.column())),
            Down => self
                .row()
                .checked_add(1)
                .map(|row| Location::new(row, *self.column())),
            Left => self
                .column()
                .checked_sub(1)
                .map(|column| Location::new(*self.row(), column)),
            Right => self
                .column()
                .checked_add(1)
                .map(|column| Location::new(*self.row(), column)),
        }
    }
}

impl From<LocationTuple> for Location {
    fn from(inner: LocationTuple) -> Self {
        Self(inner)
    }
}

impl From<Location> for LocationTuple {
    fn from(location: Location) -> Self {
        location.0
    }
}

impl AsRef<LocationTuple> for Location {
    fn as_ref(&self) -> &LocationTuple {
        self.inner()
    }
}

impl AsMut<LocationTuple> for Location {
    fn as_mut(&mut self) -> &mut LocationTuple {
        self.inner_mut()
    }
}
