use core::marker::PhantomData;

use rust_gpu_bridge::glam::Vec4;

use crate::prelude::Field;

use super::Attribute;

#[repr(C)]
pub struct Color<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Default for Color<Dim> {
    fn default() -> Self {
        Color {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Clone for Color<Dim> {
    fn clone(&self) -> Self {
        Color {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Dim> Copy for Color<Dim> {}

impl<Dim> Attribute for Color<Dim> {
    type Input = Dim;
    type Output = Vec4;
}

impl<Dim> Field<Color<Dim>> for Vec4 {
    fn field(&self, _: Dim) -> Vec4 {
        *self
    }
}
