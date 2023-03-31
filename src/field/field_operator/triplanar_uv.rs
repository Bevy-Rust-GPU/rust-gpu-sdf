use rust_gpu_bridge::{
    glam::{Vec3, Vec3Swizzles},
    Pow,
};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, Normal, Tangent, Uv},
};

use super::{FieldOperator, Operator};

/// Apply triplanar UV mapping to the provided SDF
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
pub struct TriplanarUvOp {
    pub k: f32,
}

impl<Sdf> FieldOperator<Sdf, Vec3, Uv> for TriplanarUvOp
where
    Sdf: Field<Vec3, Normal<Vec3>>,
{
    fn operator(&self, _: Uv, sdf: &Sdf, p: Vec3) -> <Uv as crate::prelude::Attribute>::Type {
        let front = p.xy();
        let side = p.zy();
        let top = p.xz();

        let weights = sdf
            .field(Normal::<Vec3>::default(), p)
            .abs()
            .pow(Vec3::splat(self.k))
            .normalize();

        front * weights.z + side * weights.x + top * weights.y
    }
}

impl_passthrough_op_1!(TriplanarUvOp, Distance, Dim);
impl_passthrough_op_1!(TriplanarUvOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(TriplanarUvOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(TriplanarUvOp, Color, Dim);

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
