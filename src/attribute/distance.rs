use core::{marker::PhantomData, ops::{Deref, DerefMut}};

use crate::prelude::{Field, items::position::Position};

use super::Attribute;

#[repr(C)]
pub struct AttrDistance<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Default for AttrDistance<Dim> {
    fn default() -> Self {
        AttrDistance {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Clone for AttrDistance<Dim> {
    fn clone(&self) -> Self {
        AttrDistance {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Dim> Copy for AttrDistance<Dim> {}

impl<Dim> Attribute for AttrDistance<Dim> {
    type Input = Position<Dim>;
    type Output = Distance;
}

impl<Dim> Field<AttrDistance<Dim>> for f32 {
    fn field(&self, _: &Position<Dim>) -> Distance {
        Distance(*self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Distance(pub f32);

impl Deref for Distance {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Distance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<f32> for Distance {
    fn from(value: f32) -> Self {
        Distance(value)
    }
}
