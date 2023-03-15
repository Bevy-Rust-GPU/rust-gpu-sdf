use rust_gpu_bridge::prelude::{Vec2, Vec2Swizzles, Vec3};
use type_fields::Field;

use crate::{
    default,
    prelude::{Distance, Normal, Operator, SignedDistanceField, SignedDistanceOperator},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizeOp;

impl SignedDistanceOperator<f32, Normal<f32>> for NormalizeOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: f32) -> Normal<f32>
    where
        Sdf: SignedDistanceField<f32, Normal<f32>>,
    {
        Normal((*sdf.evaluate(p)).signum())
    }
}

impl SignedDistanceOperator<Vec2, Normal<Vec2>> for NormalizeOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Normal<Vec2>
    where
        Sdf: SignedDistanceField<Vec2, Normal<Vec2>>,
    {
        Normal((*sdf.evaluate(p)).normalize())
    }
}

impl SignedDistanceOperator<Vec3, Normal<Vec3>> for NormalizeOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Normal<Vec3>
    where
        Sdf: SignedDistanceField<Vec3, Normal<Vec3>>,
    {
        Normal((*sdf.evaluate(p)).normalize())
    }
}

pub type Normalize<Sdf> = Operator<NormalizeOp, Sdf>;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct TetrahedronDerivative<Sdf> {
    pub sdf: Sdf,
    pub epsilon: f32,
}

impl<Sdf> SignedDistanceField<Vec2, Normal<Vec2>> for TetrahedronDerivative<Sdf>
where
    Sdf: SignedDistanceField<Vec2, Distance>,
{
    fn evaluate(&self, p: Vec2) -> Normal<Vec2> {
        let k = Vec2::new(1.0, -1.0);
        (k.xy() * *self.sdf.evaluate(p + k.xy() * self.epsilon)
            + k.yy() * *self.sdf.evaluate(p + k.yy() * self.epsilon)
            + k.yx() * *self.sdf.evaluate(p + k.yx() * self.epsilon)
            + k.xx() * *self.sdf.evaluate(p + k.xx() * self.epsilon))
        .into()
    }
}

impl<Sdf> SignedDistanceField<Vec3, Normal<Vec3>> for TetrahedronDerivative<Sdf>
where
    Sdf: SignedDistanceField<Vec3, Distance>,
{
    fn evaluate(&self, p: Vec3) -> Normal<Vec3> {
        let k = Vec2::new(1.0, -1.0);
        (k.xyy() * *self.sdf.evaluate(p + k.xyy() * self.epsilon)
            + k.yyx() * *self.sdf.evaluate(p + k.yyx() * self.epsilon)
            + k.yxy() * *self.sdf.evaluate(p + k.yxy() * self.epsilon)
            + k.xxx() * *self.sdf.evaluate(p + k.xxx() * self.epsilon))
        .into()
    }
}

pub type TetrahedronNormal<Sdf> = Normalize<TetrahedronDerivative<Sdf>>;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct CentralDiffDerivative<Sdf> {
    pub sdf: Sdf,
    pub epsilon: f32,
}

impl<Sdf> SignedDistanceField<f32, Normal<f32>> for CentralDiffDerivative<Sdf>
where
    Sdf: SignedDistanceField<f32, Distance>,
{
    fn evaluate(&self, p: f32) -> Normal<f32> {
        Normal(*self.sdf.evaluate(p + self.epsilon) - *self.sdf.evaluate(p - self.epsilon))
    }
}

impl<Sdf> SignedDistanceField<Vec2, Normal<Vec2>> for CentralDiffDerivative<Sdf>
where
    Sdf: SignedDistanceField<Vec2, Distance>,
{
    fn evaluate(&self, p: Vec2) -> Normal<Vec2> {
        (Vec2::new(
            *self.sdf.evaluate(Vec2::new(p.x + self.epsilon, p.y))
                - *self.sdf.evaluate(Vec2::new(p.x - self.epsilon, p.y)),
            *self.sdf.evaluate(Vec2::new(p.x, p.y + self.epsilon))
                - *self.sdf.evaluate(Vec2::new(p.x, p.y - self.epsilon)),
        ))
        .into()
    }
}

impl<Sdf> SignedDistanceField<Vec3, Normal<Vec3>> for CentralDiffDerivative<Sdf>
where
    Sdf: SignedDistanceField<Vec3, Distance>,
{
    fn evaluate(&self, p: Vec3) -> Normal<Vec3> {
        (Vec3::new(
            *self.sdf.evaluate(Vec3::new(p.x + self.epsilon, p.y, p.z))
                - *self.sdf.evaluate(Vec3::new(p.x - self.epsilon, p.y, p.z)),
            *self.sdf.evaluate(Vec3::new(p.x, p.y + self.epsilon, p.z))
                - *self.sdf.evaluate(Vec3::new(p.x, p.y - self.epsilon, p.z)),
            *self.sdf.evaluate(Vec3::new(p.x, p.y, p.z + self.epsilon))
                - *self.sdf.evaluate(Vec3::new(p.x, p.y, p.z - self.epsilon)),
        ))
        .into()
    }
}

pub type CentralDiffNormal<Sdf> = Normalize<CentralDiffDerivative<Sdf>>;

impl<Sdf> CentralDiffNormal<Sdf> {
    pub fn new(sdf: Sdf, epsilon: f32) -> Self {
        CentralDiffNormal {
            target: CentralDiffDerivative { sdf, epsilon },
            op: default(),
        }
    }
}
