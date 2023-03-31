use rust_gpu_bridge::glam::Vec2;

use crate::prelude::Field;

use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Uv;

impl Attribute for Uv {
    type Type = Vec2;
}

impl<Dim> Field<Dim, Uv> for Vec2 {
    fn field(&self, _: Uv, _: Dim) -> Vec2 {
        *self
    }
}
