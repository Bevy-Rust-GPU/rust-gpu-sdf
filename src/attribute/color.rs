use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use rust_gpu_bridge::glam::Vec4;

use crate::{
    impl_newtype,
    prelude::{items::position::Position, Field},
};

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

impl_newtype!(
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Color(Vec4);
);
