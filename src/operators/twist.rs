//! Twist a distance field around an arbitrary axis.

use rust_gpu_bridge::prelude::{Quat, Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, SignedDistanceField};

use super::{Operator, SignedDistanceOperator};

/// Twist a distance field around an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
pub struct TwistOp<Dim> {
    pub axis_pos: Dim,
    pub axis_rot: Dim,
    pub k: f32,
}

impl Default for TwistOp<Vec2> {
    fn default() -> Self {
        TwistOp {
            axis_pos: Vec2::Y,
            axis_rot: Vec2::Y,
            k: 1.0,
        }
    }
}

impl Default for TwistOp<Vec3> {
    fn default() -> Self {
        TwistOp {
            axis_pos: Vec3::Y,
            axis_rot: Vec3::Y,
            k: 1.0,
        }
    }
}

impl SignedDistanceOperator<Vec2, Distance> for TwistOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        let q = Vec2::from_angle(self.k * self.axis_pos.dot(p)).rotate(p);
        sdf.evaluate(q)
    }
}

impl SignedDistanceOperator<Vec3, Distance> for TwistOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        let q = Quat::from_axis_angle(self.axis_rot, self.k * self.axis_pos.dot(p)) * p;
        sdf.evaluate(q)
    }
}

/// Twist a distance field around an arbitrary axis.
pub type Twist<Dim, Sdf> = Operator<TwistOp<Dim>, Sdf>;

impl<Dim, Sdf> Twist<Dim, Sdf> {
    pub fn axis_pos(&mut self) -> &mut Dim {
        &mut self.op.axis_pos
    }

    pub fn axis_rot(&mut self) -> &mut Dim {
        &mut self.op.axis_rot
    }

    pub fn k(&mut self) -> &mut f32 {
        &mut self.op.k
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Torus;

    use super::Twist;

    #[test]
    fn test_twist() {
        Twist::<_, Torus>::default()
            .with(Twist::axis_pos, Vec3::default())
            .with(Twist::axis_rot, Vec3::default())
            .with(Twist::k, f32::default());
    }
}
