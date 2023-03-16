//! Rotate a distance field.
use core::fmt::Debug;

use rust_gpu_bridge::prelude::{Quat, Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct Rotate2dOp {
    pub angle: f32,
}

impl SignedDistanceOperator<Vec2, Distance> for Rotate2dOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        sdf.evaluate(Vec2::from_angle(-self.angle).rotate(p))
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

impl SignedDistanceOperator<Vec3, Distance> for Rotate3dOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        sdf.evaluate(self.rotation.inverse() * p)
    }
}

/// Rotate a distance field.
pub type Rotate3d<Sdf> = Operator<Rotate3dOp, Sdf>;

impl<Sdf> Rotate3d<Sdf> {
    pub fn rotation(&mut self) -> &mut Quat {
        &mut self.op.rotation
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Quat;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Square};

    use super::{Rotate2d, Rotate3d};

    #[test]
    fn test_rotate_2d() {
        Rotate2d::<Square>::default().with(Rotate2d::angle, f32::default());
    }

    #[test]
    fn test_rotate_3d() {
        Rotate3d::<Cube>::default().with(Rotate3d::rotation, Quat::default());
    }
}
