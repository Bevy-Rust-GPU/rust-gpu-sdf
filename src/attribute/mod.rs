//! Attributes whose corresponding data type can be evaluated via field function

pub trait Attribute {
    type Input;
    type Output;
}

/// A list of `Attribute`s
///
/// Extension trait of Attribute;
/// applied over `(LHS, RHS)` and `(LHS, ())` to recurse
/// through arbitrarly-long cons list impls.
pub trait Attributes {
    type Input;
    type Output;
}

impl<LHS, RHS> Attributes for (LHS, RHS)
where
    LHS: Attribute,
    RHS: Attributes<Input = LHS::Input>,
{
    type Input = LHS::Input;
    type Output = (LHS::Output, RHS::Output);
}

impl<LHS> Attributes for (LHS, ())
where
    LHS: Attribute,
{
    type Input = LHS::Input;
    type Output = (LHS::Output, ());
}

pub mod color;
pub mod distance;
pub mod normal;
pub mod tangent;
pub mod uv;

pub mod support_function;

pub mod bound_error;
