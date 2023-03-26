use rust_gpu_bridge::{
    glam::{Vec3, Vec3Swizzles},
    Pow,
};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, FieldFunction, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct TriplanarUvOp {
    pub k: f32,
}

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for TriplanarUvOp
where
    Sdf: FieldFunction<Vec3, Normal<Vec3>>,
{
    fn operator(&self, _: Uv, sdf: &Sdf, p: Vec3) -> <Uv as crate::prelude::Attribute>::Type {
        let front = p.xy();
        let side = p.zy();
        let top = p.xz();

        let weights = sdf
            .evaluate(Normal::<Vec3>::default(), p)
            .abs()
            .pow(Vec3::splat(self.k))
            .normalize();

        front * weights.z + side * weights.x + top * weights.y
    }
}

impl_passthrough_op_1!(TriplanarUvOp, <Dim>, Distance);
impl_passthrough_op_1!(TriplanarUvOp, <Dim>, Normal<Dim>);
impl_passthrough_op_1!(TriplanarUvOp, <Dim>, Tangent<Dim>);
impl_passthrough_op_1!(TriplanarUvOp, <Dim>, Color);

pub type TriplanarUv<Sdf> = Operator<TriplanarUvOp, Sdf>;

impl<Sdf> TriplanarUv<Sdf> {
    pub fn k(&mut self) -> &mut f32 {
        self.op().k()
    }
}
