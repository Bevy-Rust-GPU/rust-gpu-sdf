use core::marker::PhantomData;

use rust_gpu_bridge::glam::Vec2;

use crate::prelude::Field;

use super::Attribute;

#[repr(C)]
pub struct Uv<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Default for Uv<Dim> {
    fn default() -> Self {
        Uv {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Clone for Uv<Dim> {
    fn clone(&self) -> Self {
        Uv {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Dim> Copy for Uv<Dim> {}

impl<Dim> Attribute for Uv<Dim> {
    type Input = Dim;
    type Output = Vec2;
}

impl<Input> Field<Uv<Input>> for Vec2 {
    fn field(&self, _: &Input) -> Vec2 {
        *self
    }
}
