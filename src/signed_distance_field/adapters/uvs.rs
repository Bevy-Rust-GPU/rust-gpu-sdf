use rust_gpu_bridge::{
    glam::{Vec2, Vec3, Vec3Swizzles},
    Pow,
};
use type_fields::Field;

use crate::signed_distance_field::{
    attributes::{distance::Distance, normal::Normal, uv::Uv},
    DistanceFunction,
};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct TriplanarUvs<Sdf> {
    pub sdf: Sdf,
    pub k: f32,
}

impl<Sdf, Dim> DistanceFunction<Dim, Distance> for TriplanarUvs<Sdf>
where
    Sdf: DistanceFunction<Dim, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Dim) -> f32 {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> DistanceFunction<Dim, Normal<Dim>> for TriplanarUvs<Sdf>
where
    Sdf: DistanceFunction<Dim, Normal<Dim>>,
{
    fn evaluate(&self, attr: Normal<Dim>, p: Dim) -> Dim {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf> DistanceFunction<Vec3, Uv> for TriplanarUvs<Sdf>
where
    Sdf: DistanceFunction<Vec3, Normal<Vec3>>,
{
    fn evaluate(&self, attr: Uv, p: Vec3) -> Vec2 {
        let front = p.xy();
        let side = p.zy();
        let top = p.xz();

        let weights = self
            .sdf
            .evaluate(Normal::<Vec3>::default(), p)
            .abs()
            .pow(Vec3::splat(self.k))
            .normalize();

        front * weights.z + side * weights.x + top * weights.y
    }
}

/// Override the UVs of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfUvs<SdfA, SdfB> {
    pub sdf_base: SdfA,
    pub sdf_uv: SdfB,
}

impl<SdfA, SdfB, In> DistanceFunction<In, Distance> for SdfUvs<SdfA, SdfB>
where
    SdfA: DistanceFunction<In, Distance>,
{
    fn evaluate(&self, attr: Distance, p: In) -> f32 {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> DistanceFunction<In, Normal<In>> for SdfUvs<SdfA, SdfB>
where
    SdfA: DistanceFunction<In, Normal<In>>,
{
    fn evaluate(&self, attr: Normal<In>, p: In) -> In {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> DistanceFunction<In, Uv> for SdfUvs<SdfA, SdfB>
where
    SdfB: DistanceFunction<In, Uv>,
{
    fn evaluate(&self, attr: Uv, p: In) -> Vec2 {
        self.sdf_uv.evaluate(attr, p)
    }
}
