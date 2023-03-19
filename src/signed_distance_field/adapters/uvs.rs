use rust_gpu_bridge::prelude::{Pow, Vec3, Vec3Swizzles};
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
    fn evaluate(&self, p: Dim) -> Distance {
        self.sdf.evaluate(p)
    }
}

impl<Sdf, Dim> DistanceFunction<Dim, Normal<Dim>> for TriplanarUvs<Sdf>
where
    Sdf: DistanceFunction<Dim, Normal<Dim>>,
{
    fn evaluate(&self, p: Dim) -> Normal<Dim> {
        self.sdf.evaluate(p)
    }
}

impl<Sdf> DistanceFunction<Vec3, Uv> for TriplanarUvs<Sdf>
where
    Sdf: DistanceFunction<Vec3, Normal<Vec3>>,
{
    fn evaluate(&self, p: Vec3) -> Uv {
        let front = p.xy();
        let side = p.zy();
        let top = p.xz();

        let weights = self
            .sdf
            .evaluate(p)
            .abs()
            .pow(Vec3::splat(self.k))
            .normalize();

        (front * weights.z + side * weights.x + top * weights.y).into()
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
    fn evaluate(&self, p: In) -> Distance {
        self.sdf_base.evaluate(p)
    }
}

impl<SdfA, SdfB, In> DistanceFunction<In, Normal<In>> for SdfUvs<SdfA, SdfB>
where
    SdfA: DistanceFunction<In, Normal<In>>,
{
    fn evaluate(&self, p: In) -> Normal<In> {
        self.sdf_base.evaluate(p)
    }
}

impl<SdfA, SdfB, In> DistanceFunction<In, Uv> for SdfUvs<SdfA, SdfB>
where
    SdfB: DistanceFunction<In, Uv>,
{
    fn evaluate(&self, p: In) -> Uv {
        self.sdf_uv.evaluate(p)
    }
}
