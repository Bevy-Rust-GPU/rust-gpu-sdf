use rust_gpu_bridge::glam::{Vec2, Vec2Swizzles, Vec3};
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
pub struct GradientTetrahedronOp {
    pub epsilon: f32,
}

impl Default for GradientTetrahedronOp {
    fn default() -> Self {
        GradientTetrahedronOp { epsilon: 0.001 }
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<f32>> for GradientTetrahedronOp
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

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec2>> for GradientTetrahedronOp
where
    Sdf: Field<AttrDistance<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Vec2>,
    ) -> <AttrNormal<Vec2> as crate::prelude::Attribute>::Output {
        let k = Vec2::new(1.0, -1.0);
        (k.xy() * *sdf.field(&(**p + k.xy() * self.epsilon).into())
            + k.yy() * *sdf.field(&(**p + k.yy() * self.epsilon).into())
            + k.yx() * *sdf.field(&(**p + k.yx() * self.epsilon).into())
            + k.xx() * *sdf.field(&(**p + k.xx() * self.epsilon).into()))
        .into()
    }
}

impl<Sdf> FieldOperator<Sdf, AttrNormal<Vec3>> for GradientTetrahedronOp
where
    Sdf: Field<AttrDistance<Vec3>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: &Position<Vec3>,
    ) -> <AttrNormal<Vec3> as crate::prelude::Attribute>::Output {
        let k = Vec2::new(1.0, -1.0);
        (k.xyy() * *sdf.field(&(**p + k.xyy() * self.epsilon).into())
            + k.yyx() * *sdf.field(&(**p + k.yyx() * self.epsilon).into())
            + k.yxy() * *sdf.field(&(**p + k.yxy() * self.epsilon).into())
            + k.xxx() * *sdf.field(&(**p + k.xxx() * self.epsilon).into()))
        .into()
    }
}

impl_passthrough_op_1!(GradientTetrahedronOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(GradientTetrahedronOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(GradientTetrahedronOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(GradientTetrahedronOp, AttrColor<Dim>, Dim);
impl_passthrough_op_1!(GradientTetrahedronOp, Raycast,);

pub type GradientTetrahedron<Sdf> = Operator<GradientTetrahedronOp, Sdf>;

impl<Sdf> GradientTetrahedron<Sdf> {
    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }
}

pub type NormalTetrahedron<Sdf> = Normalize<GradientTetrahedron<Sdf>>;

impl<Sdf> NormalTetrahedron<Sdf> {
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

    use super::GradientTetrahedron;

    #[test]
    fn test_gradient_tetrahedron() {
        GradientTetrahedron::<Point>::default();
    }

    test_op_attrs!(GradientTetrahedron::<Point>);
}
