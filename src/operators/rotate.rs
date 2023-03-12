//! Rotate a distance field.
use rust_gpu_bridge::prelude::{Quat, Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
pub struct Rotate2dOp {
    pub angle: f32,
}

impl SignedDistanceOperator<Vec2> for Rotate2dOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        sdf.distance(Vec2::from_angle(-self.angle).rotate(p))
    }
}

/// Rotate a 3D distance field.
pub type Rotate2d<Sdf> = Operator<Sdf, Rotate2dOp>;

#[allow(non_camel_case_types)]
pub type Rotate2d_Angle = (crate::operators::Operator_Op, Rotate2dOp_Angle);

impl<Sdf> Rotate2d<Sdf> {
    pub const ANGLE: Rotate2d_Angle = (Operator::<(), ()>::OP, Rotate2dOp::ANGLE);
}

/// Rotate a distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
pub struct Rotate3dOp {
    pub rotation: Quat,
}

impl SignedDistanceOperator<Vec3> for Rotate3dOp {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        sdf.distance(self.rotation.inverse() * p)
    }
}

/// Rotate a distance field.
pub type Rotate3d<Sdf> = Operator<Sdf, Rotate3dOp>;

#[allow(non_camel_case_types)]
pub type Rotate3d_Rotation = (crate::operators::Operator_Op, Rotate3dOp_Rotation);

impl<Sdf> Rotate3d<Sdf> {
    pub const ROTATION: Rotate3d_Rotation = (Operator::<(), ()>::OP, Rotate3dOp::ROTATION);
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Quat;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Square};

    use super::{Rotate2d, Rotate3d};

    #[test]
    fn test_rotate_2d() {
        Rotate2d::<Square>::default().with(Rotate2d::<()>::ANGLE, f32::default());
    }

    #[test]
    fn test_rotate_3d() {
        Rotate3d::<Cube>::default().with(Rotate3d::<()>::ROTATION, Quat::default());
    }
}
