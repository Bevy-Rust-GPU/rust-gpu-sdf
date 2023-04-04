use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use rust_gpu_bridge::glam::Vec4;

use crate::prelude::{items::position::Position, Field};

use super::Attribute;

#[repr(C)]
pub struct AttrColor<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Default for AttrColor<Dim> {
    fn default() -> Self {
        AttrColor {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Clone for AttrColor<Dim> {
    fn clone(&self) -> Self {
        AttrColor {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Dim> Copy for AttrColor<Dim> {}

impl<Dim> Attribute for AttrColor<Dim> {
    type Input = Position<Dim>;
    type Output = Color;
}

impl<Input> Field<AttrColor<Input>> for Vec4 {
    fn field(&self, _: &Position<Input>) -> Color {
        Color(*self)
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Color(pub Vec4);

impl Deref for Color {
    type Target = Vec4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec4> for Color {
    fn from(value: Vec4) -> Self {
        Color(value)
    }
}
