use super::Attribute;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Distance;

impl Attribute for Distance {
    type Type = f32;
}
