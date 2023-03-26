use rust_gpu_bridge::glam::{Vec2, Vec2Swizzles, Vec3};
use type_fields::Field;

use crate::{
    default,
    prelude::{Distance, FieldFunction, Normal, Normalize},
    signed_distance_field::attributes::uv::Uv,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct TetrahedronGradient<Sdf> {
    pub sdf: Sdf,
    pub epsilon: f32,
}

impl<Sdf> Default for TetrahedronGradient<Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        TetrahedronGradient {
            sdf: default(),
            epsilon: f32::EPSILON,
        }
    }
}

impl<Sdf, Dim> FieldFunction<Dim, Distance> for TetrahedronGradient<Sdf>
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Dim) -> f32 {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldFunction<Vec2, Normal<Vec2>> for TetrahedronGradient<Sdf>
where
    Sdf: FieldFunction<Vec2, Distance>,
{
    fn evaluate(&self, attr: Normal<Vec2>, p: Vec2) -> Vec2 {
        let k = Vec2::new(1.0, -1.0);
        k.xy() * self.sdf.evaluate(Distance, p + k.xy() * self.epsilon)
            + k.yy() * self.sdf.evaluate(Distance, p + k.yy() * self.epsilon)
            + k.yx() * self.sdf.evaluate(Distance, p + k.yx() * self.epsilon)
            + k.xx() * self.sdf.evaluate(Distance, p + k.xx() * self.epsilon)
    }
}

impl<Sdf> FieldFunction<Vec3, Normal<Vec3>> for TetrahedronGradient<Sdf>
where
    Sdf: FieldFunction<Vec3, Distance>,
{
    fn evaluate(&self, attr: Normal<Vec3>, p: Vec3) -> Vec3 {
        let k = Vec2::new(1.0, -1.0);
        k.xyy() * self.sdf.evaluate(Distance, p + k.xyy() * self.epsilon)
            + k.yyx() * self.sdf.evaluate(Distance, p + k.yyx() * self.epsilon)
            + k.yxy() * self.sdf.evaluate(Distance, p + k.yxy() * self.epsilon)
            + k.xxx() * self.sdf.evaluate(Distance, p + k.xxx() * self.epsilon)
    }
}

pub type TetrahedronNormal<Sdf> = Normalize<TetrahedronGradient<Sdf>>;

impl<Sdf> TetrahedronNormal<Sdf> {
    pub fn sdf(&mut self) -> &mut Sdf {
        &mut self.target.sdf
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        &mut self.target.epsilon
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct CentralDiffGradient<Sdf> {
    pub sdf: Sdf,
    pub epsilon: f32,
}

impl<Sdf> Default for CentralDiffGradient<Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        CentralDiffGradient {
            sdf: default(),
            epsilon: f32::EPSILON,
        }
    }
}

impl<Sdf> FieldFunction<f32, Normal<f32>> for CentralDiffGradient<Sdf>
where
    Sdf: FieldFunction<f32, Distance>,
{
    fn evaluate(&self, attr: Normal<f32>, p: f32) -> f32 {
        self.sdf.evaluate(Distance, p + self.epsilon)
            - self.sdf.evaluate(Distance, p - self.epsilon)
    }
}

impl<Sdf, Dim> FieldFunction<Dim, Distance> for CentralDiffGradient<Sdf>
where
    Sdf: FieldFunction<Dim, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Dim) -> f32 {
        self.sdf.evaluate(attr, p)
    }
}

impl<Sdf> FieldFunction<Vec2, Normal<Vec2>> for CentralDiffGradient<Sdf>
where
    Sdf: FieldFunction<Vec2, Distance>,
{
    fn evaluate(&self, attr: Normal<Vec2>, p: Vec2) -> Vec2 {
        (Vec2::new(
            self.sdf
                .evaluate(Distance, Vec2::new(p.x + self.epsilon, p.y))
                - self
                    .sdf
                    .evaluate(Distance, Vec2::new(p.x - self.epsilon, p.y)),
            self.sdf
                .evaluate(Distance, Vec2::new(p.x, p.y + self.epsilon))
                - self
                    .sdf
                    .evaluate(Distance, Vec2::new(p.x, p.y - self.epsilon)),
        ))
        .into()
    }
}

impl<Sdf> FieldFunction<Vec3, Normal<Vec3>> for CentralDiffGradient<Sdf>
where
    Sdf: FieldFunction<Vec3, Distance>,
{
    fn evaluate(&self, attr: Normal<Vec3>, p: Vec3) -> Vec3 {
        (Vec3::new(
            self.sdf
                .evaluate(Distance, Vec3::new(p.x + self.epsilon, p.y, p.z))
                - self
                    .sdf
                    .evaluate(Distance, Vec3::new(p.x - self.epsilon, p.y, p.z)),
            self.sdf
                .evaluate(Distance, Vec3::new(p.x, p.y + self.epsilon, p.z))
                - self
                    .sdf
                    .evaluate(Distance, Vec3::new(p.x, p.y - self.epsilon, p.z)),
            self.sdf
                .evaluate(Distance, Vec3::new(p.x, p.y, p.z + self.epsilon))
                - self
                    .sdf
                    .evaluate(Distance, Vec3::new(p.x, p.y, p.z - self.epsilon)),
        ))
        .into()
    }
}

pub type CentralDiffNormal<Sdf> = Normalize<CentralDiffGradient<Sdf>>;

impl<Sdf> CentralDiffNormal<Sdf> {
    pub fn sdf(&mut self) -> &mut Sdf {
        &mut self.target.sdf
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        &mut self.target.epsilon
    }
}

impl<Sdf> CentralDiffNormal<Sdf> {
    pub fn new(sdf: Sdf, epsilon: f32) -> Self {
        CentralDiffNormal {
            target: CentralDiffGradient { sdf, epsilon },
            op: default(),
        }
    }
}

/// Override the normals of an SDF with the normals of another SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SdfNormals<SdfA, SdfB> {
    sdf_base: SdfA,
    sdf_normals: SdfB,
}

impl<SdfA, SdfB, In> FieldFunction<In, Distance> for SdfNormals<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Distance>,
{
    fn evaluate(&self, attr: Distance, p: In) -> f32 {
        self.sdf_base.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Normal<In>> for SdfNormals<SdfA, SdfB>
where
    SdfB: FieldFunction<In, Normal<In>>,
{
    fn evaluate(&self, attr: Normal<In>, p: In) -> In {
        self.sdf_normals.evaluate(attr, p)
    }
}

impl<SdfA, SdfB, In> FieldFunction<In, Uv> for SdfNormals<SdfA, SdfB>
where
    SdfA: FieldFunction<In, Uv>,
{
    fn evaluate(&self, attr: Uv, p: In) -> Vec2 {
        self.sdf_base.evaluate(attr, p)
    }
}
