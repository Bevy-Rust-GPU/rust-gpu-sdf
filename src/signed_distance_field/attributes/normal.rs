use core::marker::PhantomData;

use crate::default;

use super::Attribute;

#[repr(C)]
pub struct Normal<Dim>(PhantomData<Dim>);

impl<Dim> Default for Normal<Dim> {
    fn default() -> Self {
        Normal(default())
    }
}

impl<Dim> Clone for Normal<Dim> {
    fn clone(&self) -> Self {
        Normal(self.0.clone())
    }
}

impl<Dim> Copy for Normal<Dim> {}

impl<Dim> Attribute for Normal<Dim> {
    type Type = Dim;
}
