//! Twist a distance field around an arbitrary axis.

use rust_gpu_bridge::glam::{Quat, Vec2, Vec3};
use type_fields::Field;

use crate::prelude::DistanceFunction;

use super::{Operator, SignedDistanceOperator};

/// Twist a distance field around an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[repr(C)]
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

impl<Sdf, Out> SignedDistanceOperator<Sdf, Vec2, Out> for TwistOp<Vec2>
where
    Sdf: DistanceFunction<Vec2, Out>,
{
    fn operator(&self, sdf: &Sdf, p: Vec2) -> Out {
        let q = Vec2::from_angle(self.k * self.axis_pos.dot(p)).rotate(p);
        sdf.evaluate(q)
    }
}

impl<Sdf, Out> SignedDistanceOperator<Sdf, Vec3, Out> for TwistOp<Vec3>
where
    Sdf: DistanceFunction<Vec3, Out>,
{
    fn operator(&self, sdf: &Sdf, p: Vec3) -> Out {
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

#[cfg(all(not(feature = "spirv-std"), test))]
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
