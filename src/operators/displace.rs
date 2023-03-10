//! Displace the output of a distance field using the output of another distance field.

use rust_gpu_bridge::prelude::Vec3;

use crate::operators::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Displace the output of a distance field using the output of another distance field.
pub struct DisplaceOp<Sdf>
where
    Sdf: SignedDistanceField<Vec3>,
{
    pub displace: Sdf,
}

impl<SdfB> SignedDistanceOperator<Vec3> for DisplaceOp<SdfB>
where
    SdfB: SignedDistanceField<Vec3>,
{
    fn operator<SdfA>(&self, sdf: SdfA, p: Vec3) -> f32
    where
        SdfA: SignedDistanceField<Vec3>,
    {
        sdf.distance(p) + self.displace.distance(p)
    }
}

/// Displace the output of a distance field using the output of another distance field.
pub type Displace<SdfA, SdfB> = Operator<SdfA, DisplaceOp<SdfB>, Vec3>;

