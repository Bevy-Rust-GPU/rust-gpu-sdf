use core::marker::PhantomData;

use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Tangent<Dim>(PhantomData<Dim>);

impl<Dim> Attribute for Tangent<Dim> {
    type Type = Dim;
}
