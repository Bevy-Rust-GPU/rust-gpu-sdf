use rust_gpu_bridge::{
    glam::{Vec3, Vec3Swizzles},
    Pow,
};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction, Normal, Uv};

use super::{FieldOperator, Operator};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct TriplanarUvOp {
    pub k: f32,
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Distance> for TriplanarUvOp
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn operator(
        &self,
        attr: Distance,
        sdf: &Sdf,
        p: Dim,
    ) -> <Distance as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Normal<Dim>> for TriplanarUvOp
where
    Sdf: FieldFunction<Dim, Normal<Dim>>,
{
    fn operator(
        &self,
        attr: Normal<Dim>,
        sdf: &Sdf,
        p: Dim,
    ) -> <Normal<Dim> as crate::prelude::Attribute>::Type {
        sdf.evaluate(attr, p)
    }
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

pub type TriplanarUv<Sdf> = Operator<TriplanarUvOp, Sdf>;

impl<Sdf> TriplanarUv<Sdf> {
    pub fn k(&mut self) -> &mut f32 {
        self.op().k()
    }
}
