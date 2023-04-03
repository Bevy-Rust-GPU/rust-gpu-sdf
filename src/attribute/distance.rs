use core::marker::PhantomData;

use crate::prelude::Field;

use super::Attribute;

#[repr(C)]
pub struct Distance<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Default for Distance<Dim> {
    fn default() -> Self {
        Distance {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Clone for Distance<Dim> {
    fn clone(&self) -> Self {
        Distance {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Dim> Copy for Distance<Dim> {}

impl<Dim> Attribute for Distance<Dim> {
    type Input = Dim;
    type Output = f32;
}

impl<Input> Field<Distance<Input>> for f32 {
    fn field(&self, _: &Input) -> f32 {
        *self
    }
}
