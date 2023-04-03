use rust_gpu_bridge::glam::{Vec2, Vec2Swizzles, Vec3};
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
pub struct GradientTetrahedronOp {
    pub epsilon: f32,
}

impl Default for GradientTetrahedronOp {
    fn default() -> Self {
        GradientTetrahedronOp { epsilon: 0.001 }
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<f32>> for GradientTetrahedronOp
where
    Sdf: Field<Distance<f32>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: f32,
    ) -> <Normal<f32> as crate::prelude::Attribute>::Output {
        sdf.field(p + self.epsilon)
            - sdf.field(p - self.epsilon)
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec2>> for GradientTetrahedronOp
where
    Sdf: Field<Distance<Vec2>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: Vec2,
    ) -> <Normal<Vec2> as crate::prelude::Attribute>::Output {
        let k = Vec2::new(1.0, -1.0);
        k.xy() * sdf.field(p + k.xy() * self.epsilon)
            + k.yy() * sdf.field(p + k.yy() * self.epsilon)
            + k.yx() * sdf.field(p + k.yx() * self.epsilon)
            + k.xx() * sdf.field(p + k.xx() * self.epsilon)
    }
}

impl<Sdf> FieldOperator<Sdf, Normal<Vec3>> for GradientTetrahedronOp
where
    Sdf: Field<Distance<Vec3>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        p: Vec3,
    ) -> <Normal<Vec3> as crate::prelude::Attribute>::Output {
        let k = Vec2::new(1.0, -1.0);
        k.xyy() * sdf.field(p + k.xyy() * self.epsilon)
            + k.yyx() * sdf.field(p + k.yyx() * self.epsilon)
            + k.yxy() * sdf.field(p + k.yxy() * self.epsilon)
            + k.xxx() * sdf.field(p + k.xxx() * self.epsilon)
    }
}

impl_passthrough_op_1!(GradientTetrahedronOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(GradientTetrahedronOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(GradientTetrahedronOp, Uv<Dim>, Dim);
impl_passthrough_op_1!(GradientTetrahedronOp, Color<Dim>, Dim);
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
