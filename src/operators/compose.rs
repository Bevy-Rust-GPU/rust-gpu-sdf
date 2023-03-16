use rust_gpu_bridge::prelude::{Vec2, Vec3, Vec3Swizzles};
use type_fields::Field;

use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

use super::{Operator, SignedDistanceOperator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct ComposeOp<SdfA, SdfB> {
    pub sdf_a: SdfA,
    pub sdf_b: SdfB,
}

impl<SdfA, SdfB> SignedDistanceOperator<Vec2, Distance> for ComposeOp<SdfA, SdfB>
where
    SdfA: SignedDistanceField<f32, Distance>,
    SdfB: SignedDistanceField<f32, Distance>,
{
    fn operator<Sdf>(&self, sdf: &Sdf, mut p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        p.x += *self.sdf_a.evaluate(p.x);
        p.y += *self.sdf_b.evaluate(p.y);
        sdf.evaluate(p)
    }
}

impl<SdfA, SdfB> SignedDistanceOperator<Vec3, Distance> for ComposeOp<SdfA, SdfB>
where
    SdfA: SignedDistanceField<Vec2, Distance>,
    SdfB: SignedDistanceField<f32, Distance>,
{
    fn operator<Sdf>(&self, sdf: &Sdf, mut p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        p.x += *self.sdf_a.evaluate(p.xy());
        p.y += *self.sdf_b.evaluate(p.z);
        sdf.evaluate(p)
    }
}

pub type Compose<Sdf, SdfA, SdfB> = Operator<ComposeOp<SdfA, SdfB>, Sdf>;

impl<Sdf, SdfA, SdfB> Compose<Sdf, SdfA, SdfB> {
    pub fn sdf_a(&mut self) -> &mut SdfA {
        &mut self.op.sdf_a
    }

    pub fn sdf_b(&mut self) -> &mut SdfB {
        &mut self.op.sdf_b
    }
}

