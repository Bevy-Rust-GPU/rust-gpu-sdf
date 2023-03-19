use crate::signed_distance_field::{
    attributes::{distance::Distance, normal::Normal, uv::Uv},
    DistanceFunction,
};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TriplanarUvs<Sdf, Dim> {
    sdf: Sdf,
    u: Dim,
    v: Dim,
}

impl<Sdf, Dim, Out> DistanceFunction<Dim, Out> for TriplanarUvs<Sdf, Dim>
where
    Sdf: DistanceFunction<Dim, Out>,
{
    fn evaluate(&self, p: Dim) -> Out {
        self.sdf.evaluate(p)
    }
}

/// Override the UVs of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SdfUvs<SdfA, SdfB> {
    sdf_base: SdfA,
    sdf_uv: SdfB,
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
