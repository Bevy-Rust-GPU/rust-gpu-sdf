//! Attributes whose corresponding data type can be evaluated via field function

pub trait Attribute {
    type Type;
}

#[cfg(feature = "glam")]
pub mod boxed {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::prelude::Attribute;

    impl<T> Attribute for Box<dyn Attribute<Type = T>> {
        type Type = T;
    }
}

pub mod color;
pub mod distance;
pub mod normal;
pub mod tangent;
pub mod uv;

pub mod support_function;
