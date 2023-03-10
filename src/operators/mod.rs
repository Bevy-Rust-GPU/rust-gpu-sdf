//! Types that modify a distance field.

pub mod composite;
pub mod displace;
pub mod elongate;
pub mod extrude;
pub mod hollow;
pub mod intersection;
pub mod reflect;
pub mod repeat;
pub mod rotate;
pub mod round;
pub mod scale;
pub mod smooth_intersection;
pub mod smooth_subtraction;
pub mod smooth_union;
pub mod subtraction;
pub mod translate;
pub mod twist;
pub mod union;

use core::marker::PhantomData;

use crate::{default, signed_distance_field::SignedDistanceField};

/// Modifies the input / output of a [`SignedDistanceField`].
pub trait SignedDistanceOperator<Dim> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
        Dim: Clone;
}

impl<T, Dim> SignedDistanceOperator<Dim> for &T
where
    T: SignedDistanceOperator<Dim>,
{
    fn operator<Sdf>(&self, sdf: Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
        Dim: Clone
    {
        <T as SignedDistanceOperator<Dim>>::operator::<Sdf>(*self, sdf, p)
    }
}

/// Applies a [`SignedDistanceOperator`] to a [`SignedDistanceField`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Operator<Sdf, Op, Dim>
where
    Sdf: SignedDistanceField<Dim>,
    Op: SignedDistanceOperator<Dim>,
{
    pub sdf: Sdf,
    pub op: Op,
    pub _phantom: PhantomData<Dim>,
}

impl<Sdf, Op, Dim> Default for Operator<Sdf, Op, Dim>
where
    Sdf: SignedDistanceField<Dim> + Default,
    Op: SignedDistanceOperator<Dim> + Default,
{
    fn default() -> Self {
        Operator {
            sdf: default(),
            op: default(),
            _phantom: default(),
        }
    }
}

impl<Sdf, Op, Dim> SignedDistanceField<Dim> for Operator<Sdf, Op, Dim>
where
    Sdf: SignedDistanceField<Dim>,
    Op: SignedDistanceOperator<Dim>,
    Dim: Clone,
{
    fn distance(&self, p: Dim) -> f32 {
        self.op.operator(&self.sdf, p)
    }
}
