//! Attributes whose corresponding data type can be evaluated via field function

use type_fields::cons::{Cons, Uncons};

pub trait Attribute {
    type Input;
    type Output;
}

/// An attribute whose input satisfies a given lifetime.
pub trait AttributeRef<'a>: Attribute<Input = Self::InputRef> {
    type InputRef: 'a;
}

impl<'a, T> AttributeRef<'a> for T
where
    T: Attribute,
    T::Input: 'a,
{
    type InputRef = T::Input;
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

/// A list of `Attribute`s whose input satisfies a given lifetime.
pub trait AttributesRef<'a>: Attributes<Input = Self::InputRef> {
    type InputRef: 'a;
}

impl<'a, T> AttributesRef<'a> for T
where
    T: Attributes,
    T::Input: 'a,
{
    type InputRef = T::Input;
}

/// A cons list of `Attribute`s
pub trait ConsAttributes<'a>: Cons<Cons = Self::ConsAttr> {
    type ConsAttr: Attributes<Input = Self::AttrInput, Output = Self::AttrOutput>;
    type AttrInput: 'a;
    type AttrOutput: Uncons<Uncons = Self::UnconsOutput>;
    type UnconsOutput;
}

impl<'a, T> ConsAttributes<'a> for T
where
    T: Cons,
    T::Cons: AttributesRef<'a>,
    <T::Cons as Attributes>::Output: Uncons,
{
    type ConsAttr = T::Cons;
    type AttrInput = <T::Cons as AttributesRef<'a>>::InputRef;
    type AttrOutput = <T::Cons as Attributes>::Output;
    type UnconsOutput = <<T::Cons as Attributes>::Output as Uncons>::Uncons;
}

pub mod color;
pub mod distance;
pub mod normal;
pub mod tangent;
pub mod uv;

pub mod support_function;

pub mod bound_error;
