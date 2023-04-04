use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use rust_gpu_bridge::glam::{Vec2, Vec3};

use crate::{
    default,
    prelude::{items::position::Position, Field},
};

use super::Attribute;

#[repr(C)]
pub struct AttrNormal<Dim>(PhantomData<Dim>);

impl<Dim> Default for AttrNormal<Dim> {
    fn default() -> Self {
        AttrNormal(default())
    }
}

impl<Dim> Clone for AttrNormal<Dim> {
    fn clone(&self) -> Self {
        AttrNormal(self.0.clone())
    }
}

impl<Dim> Copy for AttrNormal<Dim> {}

impl<Dim> Attribute for AttrNormal<Dim> {
    type Input = Position<Dim>;
    type Output = Normal<Dim>;
}

impl Field<AttrNormal<f32>> for f32 {
    fn field(&self, _: &Position<f32>) -> Normal<f32> {
        Normal(*self)
    }
}

impl Field<AttrNormal<Vec2>> for Vec2 {
    fn field(&self, _: &Position<Vec2>) -> Normal<Vec2> {
        Normal(*self)
    }
}

impl Field<AttrNormal<Vec3>> for Vec3 {
    fn field(&self, _: &Position<Vec3>) -> Normal<Vec3> {
        Normal(*self)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Normal<Dim>(pub Dim);

impl<Dim> Deref for Normal<Dim> {
    type Target = Dim;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Dim> DerefMut for Normal<Dim> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Dim> From<Dim> for Normal<Dim> {
    fn from(value: Dim) -> Self {
        Normal(value)
    }
}

