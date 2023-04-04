use core::{marker::PhantomData, ops::{DerefMut, Deref}};

use rust_gpu_bridge::glam::Vec2;

use crate::prelude::{Field, items::position::Position};

use super::Attribute;

#[repr(C)]
pub struct AttrUv<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Default for AttrUv<Dim> {
    fn default() -> Self {
        AttrUv {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Clone for AttrUv<Dim> {
    fn clone(&self) -> Self {
        AttrUv {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Dim> Copy for AttrUv<Dim> {}

impl<Dim> Attribute for AttrUv<Dim> {
    type Input = Position<Dim>;
    type Output = Uv;
}

impl<Dim> Field<AttrUv<Dim>> for Vec2 {
    fn field(&self, _: &Position<Dim>) -> Uv {
        Uv(*self)
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Uv(pub Vec2);

impl Deref for Uv {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec2> for Uv {
    fn from(value: Vec2) -> Self {
        Uv(value)
    }
}
