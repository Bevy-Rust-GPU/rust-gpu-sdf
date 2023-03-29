use rust_gpu_bridge::glam::{Vec2, Vec3};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, FieldFunction, Normal, Normalize, Tangent, Uv},
};

use super::{FieldOperator, Operator};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[repr(C)]
pub struct GradientCentralDiffOp {
    pub epsilon: f32,
}

impl Default for GradientCentralDiffOp {
    fn default() -> Self {
        GradientCentralDiffOp {
            epsilon: 0.01,
        }
    }
}

impl<Sdf> FieldOperator<Sdf, f32, Normal<f32>> for GradientCentralDiffOp
where
    Sdf: FieldFunction<f32, Distance>,
{
    fn operator(
        &self,
        _: Normal<f32>,
        sdf: &Sdf,
        p: f32,
    ) -> <Normal<f32> as crate::prelude::Attribute>::Type {
        sdf.evaluate(Distance, p + self.epsilon) - sdf.evaluate(Distance, p - self.epsilon)
    }
}

impl<Sdf> FieldOperator<Sdf, Vec2, Normal<Vec2>> for GradientCentralDiffOp
where
    Sdf: FieldFunction<Vec2, Distance>,
{
    fn operator(
        &self,
        _: Normal<Vec2>,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Normal<Vec2> as crate::prelude::Attribute>::Type {
        Vec2::new(
            sdf.evaluate(Distance, Vec2::new(p.x + self.epsilon, p.y))
                - sdf.evaluate(Distance, Vec2::new(p.x - self.epsilon, p.y)),
            sdf.evaluate(Distance, Vec2::new(p.x, p.y + self.epsilon))
                - sdf.evaluate(Distance, Vec2::new(p.x, p.y - self.epsilon)),
        )
    }
}

impl<Sdf> FieldOperator<Sdf, Vec3, Normal<Vec3>> for GradientCentralDiffOp
where
    Sdf: FieldFunction<Vec3, Distance>,
{
    fn operator(
        &self,
        _: Normal<Vec3>,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Normal<Vec3> as crate::prelude::Attribute>::Type {
        Vec3::new(
            sdf.evaluate(Distance, Vec3::new(p.x + self.epsilon, p.y, p.z))
                - sdf.evaluate(Distance, Vec3::new(p.x - self.epsilon, p.y, p.z)),
            sdf.evaluate(Distance, Vec3::new(p.x, p.y + self.epsilon, p.z))
                - sdf.evaluate(Distance, Vec3::new(p.x, p.y - self.epsilon, p.z)),
            sdf.evaluate(Distance, Vec3::new(p.x, p.y, p.z + self.epsilon))
                - sdf.evaluate(Distance, Vec3::new(p.x, p.y, p.z - self.epsilon)),
        )
    }
}

impl_passthrough_op_1!(GradientCentralDiffOp, <Dim>, Distance);
impl_passthrough_op_1!(GradientCentralDiffOp, <Dim>, Tangent<Dim>);
impl_passthrough_op_1!(GradientCentralDiffOp, <Dim>, Uv);
impl_passthrough_op_1!(GradientCentralDiffOp, <Dim>, Color);

pub type GradientCentralDiff<Sdf> = Operator<GradientCentralDiffOp, Sdf>;

impl<Sdf> GradientCentralDiff<Sdf> {
    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }
}

pub type NormalCentralDiff<Sdf> = Normalize<GradientCentralDiff<Sdf>>;

impl<Sdf> NormalCentralDiff<Sdf> {
    pub fn sdf(&mut self) -> &mut Sdf {
        self.target().target()
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        self.target().epsilon()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use crate::{prelude::Point, test_op_attrs};

    use super::GradientCentralDiff;

    #[test]
    fn test_gradient_central_diff() {
        GradientCentralDiff::<Point>::default();
    }

    test_op_attrs!(GradientCentralDiff::<Point>);
}
