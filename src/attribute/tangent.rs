use core::{marker::PhantomData, ops::{Deref, DerefMut}};

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{default, prelude::{Field, items::position::Position}};

use super::Attribute;

#[repr(C)]
pub struct AttrTangent<Dim>(PhantomData<Dim>);

impl<Dim> Default for AttrTangent<Dim> {
    fn default() -> Self {
        AttrTangent(default())
    }
}

impl<Dim> Clone for AttrTangent<Dim> {
    fn clone(&self) -> Self {
        AttrTangent(self.0.clone())
    }
}

impl<Dim> Copy for AttrTangent<Dim> {}

impl<Dim> Attribute for AttrTangent<Dim> {
    type Input = Position<Dim>;
    type Output = Tangent<Dim>;
}

impl Field<AttrTangent<f32>> for f32 {
    fn field(&self, _: &Position<f32>) -> Tangent<f32> {
        Tangent(*self)
    }
}

impl Field<AttrTangent<Vec2>> for Vec2 {
    fn field(&self, _: &Position<Vec2>) -> Tangent<Vec2> {
        Tangent(*self)
    }
}

impl Field<AttrTangent<Vec3>> for Vec3 {
    fn field(&self, _: &Position<Vec3>) -> Tangent<Vec3> {
        Tangent(*self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tangent<Dim>(pub Dim);

impl<Dim> Deref for Tangent<Dim> {
    type Target = Dim;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Dim> DerefMut for Tangent<Dim> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Dim> From<Dim> for Tangent<Dim> {
    fn from(value: Dim) -> Self {
        Tangent(value)
    }
}

