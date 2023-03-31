use core::marker::PhantomData;

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{default, prelude::FieldFunction};

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
    type Type = Dim;
}

impl<Dim> FieldFunction<Dim, Normal<f32>> for f32 {
    fn field(&self, _: Normal<f32>, _: Dim) -> f32 {
        *self
    }
}

impl<Dim> FieldFunction<Dim, Normal<Vec2>> for Vec2 {
    fn field(&self, _: Normal<Vec2>, _: Dim) -> Vec2 {
        *self
    }
}

impl<Dim> FieldFunction<Dim, Normal<Vec3>> for Vec3 {
    fn field(&self, _: Normal<Vec3>, _: Dim) -> Vec3 {
        *self
    }
}
