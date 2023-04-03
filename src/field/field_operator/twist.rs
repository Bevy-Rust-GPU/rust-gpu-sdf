//! Twist a distance field around an arbitrary axis.

use rust_gpu_bridge::glam::{Quat, Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Attribute, Field};

use super::{FieldOperator, Operator};

/// Twist a distance field around an arbitrary axis.
#[derive(Debug, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
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

impl<Sdf, Attr> FieldOperator<Sdf, Attr> for TwistOp<Vec2>
where
    Attr: Attribute<Input = Vec2>,
    Sdf: Field<Attr>,
{
    fn operator(&self, sdf: &Sdf, input: &Vec2) -> Attr::Output {
        let q = Vec2::from_angle(self.k * self.axis_pos.dot(*input)).rotate(*input);
        sdf.field(&q)
    }
}

impl<Sdf, Attr> FieldOperator<Sdf, Attr> for TwistOp<Vec3>
where
    Attr: Attribute<Input = Vec3>,
    Sdf: Field<Attr>,
{
    fn operator(&self, sdf: &Sdf, p: &Vec3) -> Attr::Output {
        let q = Quat::from_axis_angle(self.axis_rot, self.k * self.axis_pos.dot(*p)) * *p;
        sdf.field(&q)
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
    use rust_gpu_bridge::glam::{Vec2, Vec3};
    use type_fields::field::Field;

    use crate::{
        prelude::{Point, Torus, Twist},
        test_op_attrs_2d, test_op_attrs_3d,
    };

    #[test]
    fn test_twist() {
        Twist::<_, Torus>::default()
            .with(Twist::axis_pos, Vec3::default())
            .with(Twist::axis_rot, Vec3::default())
            .with(Twist::k, f32::default());
    }

    test_op_attrs_2d!(Twist::<Vec2, Point>);
    test_op_attrs_3d!(Twist::<Vec3, Point>);
}
