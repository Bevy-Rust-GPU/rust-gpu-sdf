use rust_gpu_bridge::glam::{Vec2, Vec3, Vec4};
use type_fields::Field;

use crate::prelude::{default, Color, Distance, FieldFunction, Normal, Uv};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Copy, Clone, PartialEq, Field)]
pub struct Colorize<Sdf> {
    pub sdf: Sdf,
    pub color: Vec4,
}

impl<Sdf> Default for Colorize<Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        Colorize {
            sdf: default(),
            color: Vec4::ONE,
        }
    }
}

impl<Sdf, Dim> FieldFunction<Dim, Distance> for Colorize<Sdf>
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Dim) -> f32 {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> FieldFunction<Dim, Normal<Dim>> for Colorize<Sdf>
where
    Sdf: FieldFunction<Dim, Normal<Dim>>,
{
    fn evaluate(&self, attr: Normal<Dim>, p: Dim) -> Dim {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldFunction<Vec3, Uv> for Colorize<Sdf>
where
    Sdf: FieldFunction<Vec3, Uv>,
{
    fn evaluate(&self, attr: Uv, p: Vec3) -> Vec2 {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldFunction<Vec3, Color> for Colorize<Sdf> {
    fn evaluate(&self, _: Color, _: Vec3) -> Vec4 {
        self.color
    }
}

/// Override the colors of an SDF with the UVs of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfColor<SdfA, SdfB> {
    pub sdf_base: SdfA,
    pub sdf_color: SdfB,
}

impl<SdfA, SdfB, In> FieldFunction<In, Distance> for SdfColor<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Distance>,
{
    fn evaluate(&self, attr: Distance, p: In) -> f32 {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Normal<In>> for SdfColor<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Normal<In>>,
{
    fn evaluate(&self, attr: Normal<In>, p: In) -> In {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Uv> for SdfColor<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Uv>,
{
    fn evaluate(&self, attr: Uv, p: In) -> Vec2 {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Color> for SdfColor<SdfA, SdfB>
where
    SdfB: FieldFunction<In, Color>,
{
    fn evaluate(&self, attr: Color, p: In) -> Vec4 {
        self.sdf_color.evaluate(attr, p)
    }
}
