//! Attributes whose corresponding data type can be evaluated via field function

use type_fields::t_funk::{hlist::ToTList, tlist::ToHList};

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
pub trait ConsAttributes<'a>: ToHList<HList = Self::ConsAttr> {
    type ConsAttr: Attributes<Input = Self::AttrInput, Output = Self::AttrOutput>;
    type AttrInput: 'a;
    type AttrOutput: ToTList<TList = Self::UnconsOutput>;
    type UnconsOutput;
}

impl<'a, T> ConsAttributes<'a> for T
where
    T: ToHList,
    T::HList: AttributesRef<'a>,
    <T::HList as Attributes>::Output: ToTList,
{
    type ConsAttr = T::HList;
    type AttrInput = <T::HList as AttributesRef<'a>>::InputRef;
    type AttrOutput = <T::HList as Attributes>::Output;
    type UnconsOutput = <<T::HList as Attributes>::Output as ToTList>::TList;
}

pub mod color;
pub mod distance;
pub mod normal;
pub mod tangent;
pub mod uv;

pub mod support_function;

pub mod bound_error;
