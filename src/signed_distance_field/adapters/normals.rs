use rust_gpu_bridge::prelude::{Vec2, Vec2Swizzles, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, Normal, SignedDistanceField};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct TetrahedronNormals<Sdf> {
    pub sdf: Sdf,
    pub epsilon: f32,
}

impl<Sdf> SignedDistanceField<Vec3, Normal> for TetrahedronNormals<Sdf>
where
    Sdf: SignedDistanceField<Vec3, Distance>,
{
    fn evaluate(&self, p: Vec3) -> Normal {
        let k = Vec2::new(1.0, -1.0);
        (k.xyy() * *self.sdf.evaluate(p + k.xyy() * self.epsilon)
            + k.yyx() * *self.sdf.evaluate(p + k.yyx() * self.epsilon)
            + k.yxy() * *self.sdf.evaluate(p + k.yxy() * self.epsilon)
            + k.xxx() * *self.sdf.evaluate(p + k.xxx() * self.epsilon))
        .normalize()
        .into()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct CentralDiffNormals<Sdf> {
    pub sdf: Sdf,
    pub epsilon: f32,
}

impl<Sdf> SignedDistanceField<Vec3, Normal> for CentralDiffNormals<Sdf>
where
    Sdf: SignedDistanceField<Vec3, Distance>,
{
    fn evaluate(&self, p: Vec3) -> Normal {
        (Vec3::new(
            *self.sdf.evaluate(Vec3::new(p.x + self.epsilon, p.y, p.z))
                - *self.sdf.evaluate(Vec3::new(p.x - self.epsilon, p.y, p.z)),
            *self.sdf.evaluate(Vec3::new(p.x, p.y + self.epsilon, p.z))
                - *self.sdf.evaluate(Vec3::new(p.x, p.y - self.epsilon, p.z)),
            *self.sdf.evaluate(Vec3::new(p.x, p.y, p.z + self.epsilon))
                - *self.sdf.evaluate(Vec3::new(p.x, p.y, p.z - self.epsilon)),
        ))
        .normalize()
        .into()
    }
}

