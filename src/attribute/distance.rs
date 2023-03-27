use crate::prelude::FieldFunction;

use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Distance;

impl Attribute for Distance {
    type Type = f32;
}

impl<Dim> FieldFunction<Dim, Distance> for f32 {
    fn evaluate(&self, _: Distance, _: Dim) -> f32 {
        *self
    }
}
