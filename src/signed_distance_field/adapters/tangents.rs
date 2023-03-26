use rust_gpu_bridge::glam::{Vec2, Vec2Swizzles, Vec3, Vec4};
use type_fields::Field;

use crate::prelude::{default, Color, Distance, FieldFunction, Normal, Normalize, Tangent, Uv};

/// Calculate a 3D gradient given a 2D UV
#[derive(Copy, Clone, PartialEq, Field)]
pub struct UvGradient<Sdf> {
    pub sdf: Sdf,
    pub axis: Vec2,
    pub epsilon: f32,
}

impl<Sdf> Default for UvGradient<Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        UvGradient {
            sdf: default(),
            axis: Vec2::X,
            epsilon: f32::EPSILON,
        }
    }
}

impl<Sdf, Dim> FieldFunction<Dim, Distance> for UvGradient<Sdf>
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Dim) -> f32 {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf, Dim> FieldFunction<Dim, Normal<Dim>> for UvGradient<Sdf>
where
    Sdf: FieldFunction<Dim, Normal<Dim>>,
{
    fn evaluate(&self, attr: Normal<Dim>, p: Dim) -> Dim {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldFunction<Vec3, Uv> for UvGradient<Sdf>
where
    Sdf: FieldFunction<Vec3, Uv>,
{
    fn evaluate(&self, attr: Uv, p: Vec3) -> Vec2 {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldFunction<Vec3, Tangent<Vec3>> for UvGradient<Sdf>
where
    Sdf: FieldFunction<Vec3, Uv>,
{
    fn evaluate(&self, _: Tangent<Vec3>, p: Vec3) -> Vec3 {
        let k = Vec2::new(1.0, -1.0);
        k.xyy()
            * self
                .sdf
                .evaluate(Uv, p + k.xyy() * self.epsilon)
                .dot(self.axis)
            + k.yyx()
                * self
                    .sdf
                    .evaluate(Uv, p + k.yyx() * self.epsilon)
                    .dot(self.axis)
            + k.yxy()
                * self
                    .sdf
                    .evaluate(Uv, p + k.yxy() * self.epsilon)
                    .dot(self.axis)
            + k.xxx()
                * self
                    .sdf
                    .evaluate(Uv, p + k.xxx() * self.epsilon)
                    .dot(self.axis)
    }
}

impl<Sdf> FieldFunction<Vec3, Color> for UvGradient<Sdf>
where
    Sdf: FieldFunction<Vec3, Color>,
{
    fn evaluate(&self, attr: Color, p: Vec3) -> Vec4 {
        self.sdf.evaluate(attr, p)
    }
}

pub type UvTangent<Sdf> = Normalize<UvGradient<Sdf>>;

impl<Sdf> UvTangent<Sdf> {
    pub fn sdf(&mut self) -> &mut Sdf {
        self.target().sdf()
    }

    pub fn axis(&mut self) -> &mut Vec2 {
        self.target().axis()
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        self.target().epsilon()
    }
}

/// Override the tangents of an SDF with the tangents of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct SdfTangent<SdfA, SdfB> {
    pub sdf_base: SdfA,
    pub sdf_tangent: SdfB,
}

impl<SdfA, SdfB, Pos> FieldFunction<Pos, Distance> for SdfTangent<SdfA, SdfB>
where
    SdfA: FieldFunction<Pos, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Pos) -> f32 {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Normal<In>> for SdfTangent<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Normal<In>>,
{
    fn evaluate(&self, attr: Normal<In>, p: In) -> In {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Uv> for SdfTangent<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Uv>,
{
    fn evaluate(&self, attr: Uv, p: In) -> Vec2 {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, Pos> FieldFunction<Pos, Tangent<Vec3>> for SdfTangent<SdfA, SdfB>
where
    SdfB: FieldFunction<Pos, Tangent<Vec3>>,
{
    fn evaluate(&self, attr: Tangent<Vec3>, p: Pos) -> Vec3 {
        self.sdf_tangent.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Color> for SdfTangent<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Color>,
{
    fn evaluate(&self, attr: Color, p: In) -> Vec4 {
        self.sdf_base.evaluate(attr, p)
    }
}
