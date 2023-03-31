//! Attributes whose corresponding data type can be evaluated via field function

pub trait Attribute: Default {
    type Type;
}

/// A list of `Attribute`s
///
/// Extension trait of Attribute;
/// applied over `(LHS, RHS)` and `(LHS, ())` to recurse
/// through arbitrarly-long cons list impls.
pub trait Attributes {
    type Type;
}

impl<LHS, RHS> Attributes for (LHS, RHS)
where
    LHS: Attribute,
    RHS: Attributes,
{
    type Type = (LHS::Type, RHS::Type);
}

impl<LHS> Attributes for (LHS, ())
where
    LHS: Attribute,
{
    type Type = (LHS::Type, ());
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

pub mod bound_error;
