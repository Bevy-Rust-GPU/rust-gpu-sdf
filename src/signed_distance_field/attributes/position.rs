use rust_gpu_bridge::glam::Vec3;

use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Position;

impl Attribute for Position {
    type Type = Vec3;
}

