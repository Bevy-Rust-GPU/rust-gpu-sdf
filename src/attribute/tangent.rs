use core::marker::PhantomData;

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{default, prelude::Field};

use super::Attribute;

#[repr(C)]
pub struct Tangent<Dim>(PhantomData<Dim>);

impl<Dim> Default for Tangent<Dim> {
    fn default() -> Self {
        Tangent(default())
    }
}

impl<Dim> Clone for Tangent<Dim> {
    fn clone(&self) -> Self {
        Tangent(self.0.clone())
    }
}

impl<Dim> Copy for Tangent<Dim> {}

impl<Dim> Attribute for Tangent<Dim> {
    type Input = Dim;
    type Output = Dim;
}

impl Field<Tangent<f32>> for f32 {
    fn field(&self, _: f32) -> f32 {
        *self
    }
}

impl Field<Tangent<Vec2>> for Vec2 {
    fn field(&self, _: Vec2) -> Vec2 {
        *self
    }
}

impl Field<Tangent<Vec3>> for Vec3 {
    fn field(&self, _: Vec3) -> Vec3 {
        *self
    }
}
