use core::ops::{Deref, DerefMut};

use crate::impl_newtype;

impl_newtype!(
    #[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
    pub struct Position<Dim>(Dim);
);
