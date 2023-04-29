use rust_gpu_bridge::{
    glam::{Vec3, Vec3Swizzles},
    Pow,
};
use type_fields::macros::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{AttrColor, AttrDistance, Field, AttrNormal, AttrTangent, AttrUv, items::position::Position},
};

use super::{FieldOperator, Operator};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct TriplanarUvOp {
    pub k: f32,
}

impl<Sdf> FieldOperator<Sdf, AttrUv<Vec3>> for TriplanarUvOp
where
    Sdf: Field<AttrNormal<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Vec3>) -> <AttrUv<Vec3> as crate::prelude::Attribute>::Output {
        let front = input.xy();
        let side = input.zy();
        let top = input.xz();

        let weights = sdf
            .field(input)
            .abs()
            .pow(Vec3::splat(self.k))
            .normalize();

        (front * weights.z + side * weights.x + top * weights.y).into()
    }
}

impl_passthrough_op_1!(TriplanarUvOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(TriplanarUvOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(TriplanarUvOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(TriplanarUvOp, AttrColor<Dim>, Dim);

pub type TriplanarUv<Sdf> = Operator<TriplanarUvOp, Sdf>;

impl<Sdf> TriplanarUv<Sdf> {
    pub fn k(&mut self) -> &mut f32 {
        self.op().k()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Point, Sphere},
        test_op_attrs_3d,
    };

    use super::TriplanarUv;

    #[test]
    fn test_triplanar_uv() {
        TriplanarUv::<Sphere>::default();
    }

    test_op_attrs_3d!(TriplanarUv::<Point>);
}
