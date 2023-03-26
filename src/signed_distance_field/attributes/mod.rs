//! Attributes that can be computed by a distance field

pub trait Attribute {
    type Type;
}

pub mod color;
pub mod distance;
pub mod normal;
pub mod position;
pub mod tangent;
pub mod uv;
