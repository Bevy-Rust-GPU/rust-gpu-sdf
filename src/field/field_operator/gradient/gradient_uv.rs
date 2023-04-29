use rust_gpu_bridge::glam::{Vec2, Vec2Swizzles, Vec3};
use type_fields::macros::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv, Field,
        FieldOperator, Normalize, Operator, Raycast,
    },
};

/// Calculate a 3D gradient given a 2D UV
#[derive(Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct UvGradientOp {
    pub axis: Vec2,
    pub epsilon: f32,
}

impl Default for UvGradientOp {
    fn default() -> Self {
        UvGradientOp {
            axis: Vec2::X,
            epsilon: 0.00001,
        }
    }
}

impl<Sdf> FieldOperator<Sdf, AttrTangent<Vec3>> for UvGradientOp
where
    Sdf: Field<AttrUv<Vec3>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        input: &Position<Vec3>,
    ) -> <AttrTangent<Vec3> as crate::prelude::Attribute>::Output {
        let k = Vec2::new(1.0, -1.0);
        (k.xyy()
            * sdf
                .field(&(**input + k.xyy() * self.epsilon).into())
                .dot(self.axis)
            + k.yyx()
                * sdf
                    .field(&(**input + k.yyx() * self.epsilon).into())
                    .dot(self.axis)
            + k.yxy()
                * sdf
                    .field(&(**input + k.yxy() * self.epsilon).into())
                    .dot(self.axis)
            + k.xxx()
                * sdf
                    .field(&(**input + k.xxx() * self.epsilon).into())
                    .dot(self.axis))
        .into()
    }
}

impl_passthrough_op_1!(UvGradientOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(UvGradientOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(UvGradientOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(UvGradientOp, AttrColor<Dim>, Dim);
impl_passthrough_op_1!(UvGradientOp, Raycast,);

pub type UvGradient<Sdf> = Operator<UvGradientOp, Sdf>;

impl<Sdf> UvGradient<Sdf> {
    pub fn axis(&mut self) -> &mut Vec2 {
        self.op().axis()
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }
}

pub type UvTangent<Sdf> = Normalize<UvGradient<Sdf>>;

impl<Sdf> UvTangent<Sdf> {
    pub fn sdf(&mut self) -> &mut Sdf {
        self.target().target()
    }

    pub fn axis(&mut self) -> &mut Vec2 {
        self.target().axis()
    }

    pub fn epsilon(&mut self) -> &mut f32 {
        self.target().epsilon()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use crate::{prelude::Point, test_op_attrs};

    use super::UvGradient;

    #[test]
    fn test_gradient_tetrahedron() {
        UvGradient::<Point>::default();
    }

    test_op_attrs!(UvGradient::<Point>);
}
