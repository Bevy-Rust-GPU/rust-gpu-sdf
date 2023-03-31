//! Rotate a distance field.
use core::fmt::Debug;

use rust_gpu_bridge::glam::{Quat, Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Attribute, Field, FieldOperator, Operator};

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct Rotate2dOp {
    pub angle: f32,
}

impl<Sdf, Attr> FieldOperator<Sdf, Vec2, Attr> for Rotate2dOp
where
    Attr: Attribute,
    Sdf: Field<Vec2, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Vec2) -> Attr::Type {
        sdf.field(attr, Vec2::from_angle(-self.angle).rotate(p))
    }
}

/// Rotate a 3D distance field.
pub type Rotate2d<Sdf> = Operator<Rotate2dOp, Sdf>;

impl<Sdf> Rotate2d<Sdf> {
    pub fn angle(&mut self) -> &mut f32 {
        &mut self.op.angle
    }
}

/// Rotate a distance field.
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct Rotate3dOp {
    pub rotation: Quat,
}

#[cfg(not(feature = "spirv-std"))]
impl Debug for Rotate3dOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.rotation.fmt(f)
    }
}

impl<Sdf, Attr> FieldOperator<Sdf, Vec3, Attr> for Rotate3dOp
where
    Attr: Attribute,
    Sdf: Field<Vec3, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Vec3) -> Attr::Type {
        sdf.field(attr, self.rotation.inverse() * p)
    }
}

/// Rotate a distance field.
pub type Rotate3d<Sdf> = Operator<Rotate3dOp, Sdf>;

impl<Sdf> Rotate3d<Sdf> {
    pub fn rotation(&mut self) -> &mut Quat {
        &mut self.op.rotation
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::Quat;
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, Rotate2d, Rotate3d, Square},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_rotate_2d() {
        Rotate2d::<Square>::default().with(Rotate2d::angle, f32::default());
    }

    #[test]
    fn test_rotate_3d() {
        Rotate3d::<Cube>::default().with(Rotate3d::rotation, Quat::default());
    }

    test_op_attrs_2d!(Rotate2d::<Point>);
    test_op_attrs_3d!(Rotate3d::<Point>);
}
