use rust_gpu_bridge::glam::{Vec2, Vec3};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv, Field,
        FieldOperator, Normalize, Operator, Raycast,
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

impl<Sdf> FieldOperator<Sdf, AttrNormal<f32>> for GradientCentralDiffOp
where
    Sdf: Field<AttrDistance<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        input: &Position<f32>,
    ) -> <AttrNormal<f32> as crate::prelude::Attribute>::Output {
        (*sdf.field(&(**input + self.epsilon).into())
            - *sdf.field(&(**input - self.epsilon).into()))
        .into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec2>> for GradientCentralDiffOp
where
    Sdf: Field<AttrDistance<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        input: &Position<Vec2>,
    ) -> <AttrNormal<Vec2> as crate::prelude::Attribute>::Output {
        Vec2::new(
            *sdf.field(&Vec2::new(input.x + self.epsilon, input.y).into())
                - *sdf.field(&Vec2::new(input.x - self.epsilon, input.y).into()),
            *sdf.field(&Vec2::new(input.x, input.y + self.epsilon).into())
                - *sdf.field(&Vec2::new(input.x, input.y - self.epsilon).into()),
        ).into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec3>> for GradientCentralDiffOp
where
    Sdf: Field<AttrDistance<Vec3>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Vec3>,
    ) -> <AttrNormal<Vec3> as crate::prelude::Attribute>::Output {
        Vec3::new(
            *sdf.field(&Vec3::new(p.x + self.epsilon, p.y, p.z).into())
                - *sdf.field(&Vec3::new(p.x - self.epsilon, p.y, p.z).into()),
            *sdf.field(&Vec3::new(p.x, p.y + self.epsilon, p.z).into())
                - *sdf.field(&Vec3::new(p.x, p.y - self.epsilon, p.z).into()),
            *sdf.field(&Vec3::new(p.x, p.y, p.z + self.epsilon).into())
                - *sdf.field(&Vec3::new(p.x, p.y, p.z - self.epsilon).into()),
        ).into()
    }
}

impl_passthrough_op_1!(GradientCentralDiffOp, AttrDistance::<Dim>, Dim);
impl_passthrough_op_1!(GradientCentralDiffOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(GradientCentralDiffOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(GradientCentralDiffOp, AttrColor<Dim>, Dim);
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
