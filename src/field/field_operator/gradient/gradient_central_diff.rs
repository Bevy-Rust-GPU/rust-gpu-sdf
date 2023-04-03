use rust_gpu_bridge::glam::{Vec2, Vec3};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        Color, Distance, Field, FieldOperator, Normal, Normalize, Operator, Raycast, Tangent,
        Uv,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct GradientCentralDiffOp {
    pub epsilon: f32,
}

impl Default for GradientCentralDiffOp {
    fn default() -> Self {
        GradientCentralDiffOp { epsilon: 0.001 }
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<f32>> for GradientCentralDiffOp
where
    Sdf: Field<Distance<f32>>,
{
    fn operator(&self, sdf: &Sdf, p: f32) -> <Normal<f32> as crate::prelude::Attribute>::Output {
        sdf.field(p + self.epsilon) - sdf.field(p - self.epsilon)
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec2>> for GradientCentralDiffOp
where
    Sdf: Field<Distance<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec2) -> <Normal<Vec2> as crate::prelude::Attribute>::Output {
        Vec2::new(
            sdf.field(Vec2::new(p.x + self.epsilon, p.y))
                - sdf.field(Vec2::new(p.x - self.epsilon, p.y)),
            sdf.field(Vec2::new(p.x, p.y + self.epsilon))
                - sdf.field(Vec2::new(p.x, p.y - self.epsilon)),
        )
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec3>> for GradientCentralDiffOp
where
    Sdf: Field<Distance<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec3) -> <Normal<Vec3> as crate::prelude::Attribute>::Output {
        Vec3::new(
            sdf.field(Vec3::new(p.x + self.epsilon, p.y, p.z))
                - sdf.field(Vec3::new(p.x - self.epsilon, p.y, p.z)),
            sdf.field(Vec3::new(p.x, p.y + self.epsilon, p.z))
                - sdf.field(Vec3::new(p.x, p.y - self.epsilon, p.z)),
            sdf.field(Vec3::new(p.x, p.y, p.z + self.epsilon))
                - sdf.field(Vec3::new(p.x, p.y, p.z - self.epsilon)),
        )
    }
}

impl_passthrough_op_1!(GradientCentralDiffOp, Distance::<Dim>, Dim);
impl_passthrough_op_1!(GradientCentralDiffOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(GradientCentralDiffOp, Uv<Dim>, Dim);
impl_passthrough_op_1!(GradientCentralDiffOp, Color<Dim>, Dim);
impl_passthrough_op_1!(GradientCentralDiffOp, Raycast,);

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
