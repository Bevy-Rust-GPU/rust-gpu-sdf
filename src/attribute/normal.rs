use core::marker::PhantomData;

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{default, prelude::Field};

use super::Attribute;

#[repr(C)]
pub struct Normal<Dim>(PhantomData<Dim>);

impl<Dim> Default for Normal<Dim> {
    fn default() -> Self {
        Normal(default())
    }
}

impl<Dim> Clone for Normal<Dim> {
    fn clone(&self) -> Self {
        Normal(self.0.clone())
    }
}

impl<Dim> Copy for Normal<Dim> {}

impl<Dim> Attribute for Normal<Dim> {
    type Input = Dim;
    type Output = Dim;
}

impl Field<Normal<f32>> for f32 {
    fn field(&self, _: f32) -> f32 {
        *self
    }
}

impl Field<Normal<Vec2>> for Vec2 {
    fn field(&self, _: Vec2) -> Vec2 {
        *self
    }
}

impl Field<Normal<Vec3>> for Vec3 {
    fn field(&self, _: Vec3) -> Vec3 {
        *self
    }
}
