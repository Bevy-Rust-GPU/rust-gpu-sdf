use rust_gpu_bridge::glam::Vec2;

use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Uv;

impl Attribute for Uv {
    type Type = Vec2;
}

