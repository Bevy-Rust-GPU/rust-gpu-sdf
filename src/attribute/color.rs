use rust_gpu_bridge::glam::Vec4;

use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Color;

impl Attribute for Color {
    type Type = Vec4;
}
