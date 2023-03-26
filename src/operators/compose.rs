use rust_gpu_bridge::glam::{Vec2, Vec3, Vec3Swizzles};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction};

use super::{Operator, SignedDistanceOperator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct ComposeOp<SdfA, SdfB> {
    pub sdf_a: SdfA,
    pub sdf_b: SdfB,
}

impl<Sdf, SdfA, SdfB> SignedDistanceOperator<Sdf, Vec2, Distance> for ComposeOp<SdfA, SdfB>
where
    Sdf: FieldFunction<Vec2, Distance>,
    SdfA: FieldFunction<f32, Distance>,
    SdfB: FieldFunction<f32, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, mut p: Vec2) -> f32 {
        p.x += self.sdf_a.evaluate(attr, p.x);
        p.y += self.sdf_b.evaluate(attr, p.y);
        sdf.evaluate(attr, p)
    }
}

impl<Sdf, SdfA, SdfB> SignedDistanceOperator<Sdf, Vec3, Distance> for ComposeOp<SdfA, SdfB>
where
    Sdf: FieldFunction<Vec3, Distance>,
    SdfA: FieldFunction<Vec2, Distance>,
    SdfB: FieldFunction<f32, Distance>,
{
    fn operator(&self, attr: Distance, sdf: &Sdf, mut p: Vec3) -> f32 {
        p.x += self.sdf_a.evaluate(attr, p.xy());
        p.y += self.sdf_b.evaluate(attr, p.z);
        sdf.evaluate(attr, p)
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
