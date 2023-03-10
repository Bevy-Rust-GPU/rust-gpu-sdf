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
pub trait SignedDistanceOperator<P> {
    fn operator<Sdf>(&self, sdf: Sdf, p: P) -> f32
    where
        Sdf: SignedDistanceField<P>;
}

impl<T, P> SignedDistanceOperator<P> for &T
where
    T: SignedDistanceOperator<P>,
{
    fn operator<Sdf>(&self, sdf: Sdf, p: P) -> f32
    where
        Sdf: SignedDistanceField<P>,
    {
        <T as SignedDistanceOperator<P>>::operator::<Sdf>(*self, sdf, p)
    }
}

/// Applies a [`SignedDistanceOperator`] to a [`SignedDistanceField`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Operator<Sdf, Op, P>
where
    Sdf: SignedDistanceField<P>,
    Op: SignedDistanceOperator<P>,
{
    pub sdf: Sdf,
    pub op: Op,
    pub _phantom: PhantomData<P>,
}

impl<Sdf, Op, P> Default for Operator<Sdf, Op, P>
where
    Sdf: SignedDistanceField<P> + Default,
    Op: SignedDistanceOperator<P> + Default,
{
    fn default() -> Self {
        Operator {
            sdf: default(),
            op: default(),
            _phantom: default(),
        }
    }
}

impl<S, O, P> SignedDistanceField<P> for Operator<S, O, P>
where
    S: SignedDistanceField<P>,
    O: SignedDistanceOperator<P>,
{
    fn distance(&self, p: P) -> f32 {
        self.op.operator(&self.sdf, p)
    }
}
