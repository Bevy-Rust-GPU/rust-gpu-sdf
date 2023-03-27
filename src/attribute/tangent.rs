use core::marker::PhantomData;

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{default, prelude::FieldFunction};

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
    type Type = Dim;
}

impl<Dim> FieldFunction<Dim, Tangent<f32>> for f32 {
    fn evaluate(&self, _: Tangent<f32>, _: Dim) -> f32 {
        *self
    }
}

impl<Dim> FieldFunction<Dim, Tangent<Vec2>> for Vec2 {
    fn evaluate(&self, _: Tangent<Vec2>, _: Dim) -> Vec2 {
        *self
    }
}

impl<Dim> FieldFunction<Dim, Tangent<Vec3>> for Vec3 {
    fn evaluate(&self, _: Tangent<Vec3>, _: Dim) -> Vec3 {
        *self
    }
}
