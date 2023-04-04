use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Position<Dim>(pub Dim);

impl<Dim> Deref for Position<Dim> {
    type Target = Dim;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Dim> DerefMut for Position<Dim> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Dim> Borrow<Dim> for Position<Dim> {
    fn borrow(&self) -> &Dim {
        &self.0
    }
}

impl<Dim> BorrowMut<Dim> for Position<Dim> {
    fn borrow_mut(&mut self) -> &mut Dim {
        &mut self.0
    }
}

impl<Dim> From<Dim> for Position<Dim> {
    fn from(value: Dim) -> Self {
        Position(value)
    }
}

