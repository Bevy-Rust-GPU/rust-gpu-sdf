//! Attributes whose corresponding data type can be evaluated via field function

pub trait Attribute {
    type Type;
}

pub mod color;
pub mod distance;
pub mod normal;
pub mod tangent;
pub mod uv;

pub mod support_function;
