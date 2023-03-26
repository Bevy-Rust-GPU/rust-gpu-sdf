//! Attributes that can be computed by a distance field

pub trait Attribute {
    type Type;
}

pub mod distance;
pub mod position;
pub mod normal;
pub mod uv;
pub mod tangent;
pub mod color;
