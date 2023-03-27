//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Attribute, FieldFunction, FieldOperator, Operator};

pub const AXIS_X: usize = 1;
pub const AXIS_Y: usize = 2;
pub const AXIS_Z: usize = 4;

pub const AXIS_XY: usize = AXIS_X | AXIS_Y;
pub const AXIS_XYZ: usize = AXIS_XY | AXIS_Z;

/// Cheaply reflect a distance field about X / Y / Z using a const axis bitmask.
/// NOTE: Will produce a bound unless any geometry crossing
///       the reflecting planes is already a field w.r.t. its reflection.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct AxialReflectOp<const AXIS: usize>;

impl<const AXIS: usize, Sdf, Attr> FieldOperator<Sdf, f32, Attr> for AxialReflectOp<AXIS>
where
    Attr: Attribute,
    Sdf: FieldFunction<f32, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, mut p: f32) -> Attr::Type {
        if AXIS & AXIS_X > 0 {
            p = p.abs();
        }

        sdf.evaluate(attr, p)
    }
}

impl<const AXIS: usize, Sdf, Attr> FieldOperator<Sdf, Vec2, Attr> for AxialReflectOp<AXIS>
where
    Attr: Attribute,
    Sdf: FieldFunction<Vec2, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, mut p: Vec2) -> Attr::Type {
        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        sdf.evaluate(attr, p)
    }
}

impl<const AXIS: usize, Sdf, Attr> FieldOperator<Sdf, Vec3, Attr> for AxialReflectOp<AXIS>
where
    Attr: Attribute,
    Sdf: FieldFunction<Vec3, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, mut p: Vec3) -> Attr::Type {
        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            p.z = p.z.abs();
        }

        sdf.evaluate(attr, p)
    }
}

/// Reflect a distance field about X / Y / Z
pub type AxialReflect<const AXIS: usize, Sdf> = Operator<AxialReflectOp<AXIS>, Sdf>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use crate::{
        prelude::{Circle, Cube},
        test_op_attrs,
    };

    use super::{AxialReflect, AXIS_XYZ};

    #[test]
    fn test_axial_reflect() {
        AxialReflect::<AXIS_XYZ, Cube>::default();
    }

    test_op_attrs!(AxialReflect::<AXIS_XYZ, Circle>);
}
