use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use rust_gpu_bridge::glam::Vec2;

use crate::{
    impl_newtype,
    prelude::{items::position::Position, Field},
};

use super::Attribute;

#[repr(C)]
pub struct AttrUv<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Default for AttrUv<Dim> {
    fn default() -> Self {
        AttrUv {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Clone for AttrUv<Dim> {
    fn clone(&self) -> Self {
        AttrUv {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Dim> Copy for AttrUv<Dim> {}

impl<Dim> Attribute for AttrUv<Dim> {
    type Input = Position<Dim>;
    type Output = Uv;
}

impl<Dim> Field<AttrUv<Dim>> for Vec2 {
    fn field(&self, _: &Position<Dim>) -> Uv {
        Uv(*self)
    }
}

impl_newtype!(
    #[derive(Default, Copy, Clone, PartialEq)]
    pub struct Uv(Vec2);
);
