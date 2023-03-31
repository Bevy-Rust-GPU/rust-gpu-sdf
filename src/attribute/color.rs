use rust_gpu_bridge::glam::Vec4;

use crate::prelude::FieldFunction;

use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Color;

impl Attribute for Color {
    type Type = Vec4;
}

impl<Dim> FieldFunction<Dim, Color> for Vec4 {
    fn field(&self, _: Color, _: Dim) -> Vec4 {
        *self
    }
}
