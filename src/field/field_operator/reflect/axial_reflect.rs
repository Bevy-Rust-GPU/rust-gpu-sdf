//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Attribute, Distance, Field, FieldOperator, Normal, Operator, Uv};

pub const AXIS_X: usize = 1;
pub const AXIS_Y: usize = 2;
pub const AXIS_Z: usize = 4;

pub const AXIS_XY: usize = AXIS_X | AXIS_Y;
pub const AXIS_XYZ: usize = AXIS_XY | AXIS_Z;

/// Cheaply reflect a distance field about X / Y / Z using a const axis bitmask.
/// NOTE: Will produce a bound unless any geometry crossing
///       the reflecting planes is already a field w.r.t. its reflection.
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct AxialReflectOp<const AXIS: usize>;

impl<const AXIS: usize, Sdf, Attr> FieldOperator<Sdf, Attr> for AxialReflectOp<AXIS>
where
    Attr: Attribute<Input = f32>,
    Sdf: Field<Attr>,
{
    fn operator(&self, sdf: &Sdf, mut p: f32) -> Attr::Output {
        if AXIS & AXIS_X > 0 {
            p = p.abs();
        }

        sdf.field(p)
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, Distance<Vec2>> for AxialReflectOp<AXIS>
where
    Sdf: Field<Distance<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, mut p: Vec2) -> f32 {
        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        sdf.field(p)
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, Normal<Vec2>> for AxialReflectOp<AXIS>
where
    Sdf: Field<Normal<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec2) -> Vec2 {
        let mut pa = p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        let mut n = sdf.field(pa);

        if AXIS & AXIS_X > 0 && p.x < 0.0 {
            n.x *= -1.0;
        }

        if AXIS & AXIS_Y > 0 && p.y < 0.0 {
            n.y *= -1.0;
        }

        n
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, Uv<Vec2>> for AxialReflectOp<AXIS>
where
    Sdf: Field<Uv<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec2) -> Vec2 {
        let mut pa = p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        let mut n = sdf.field(pa);

        if AXIS & AXIS_X > 0 && p.x < 0.0 {
            n.x *= -1.0;
        }

        if AXIS & AXIS_Y > 0 && p.y < 0.0 {
            n.y *= -1.0;
        }

        n
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, Distance<Vec3>> for AxialReflectOp<AXIS>
where
    Sdf: Field<Distance<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, mut p: Vec3) -> f32 {
        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            p.z = p.z.abs();
        }

        sdf.field(p)
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, Normal<Vec3>> for AxialReflectOp<AXIS>
where
    Sdf: Field<Normal<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec3) -> Vec3 {
        let mut pa = p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            pa.z = p.z.abs();
        }

        let mut n = sdf.field(pa);

        if AXIS & AXIS_X > 0 && p.x < 0.0 {
            n.x *= -1.0;
        }

        if AXIS & AXIS_Y > 0 && p.y < 0.0 {
            n.y *= -1.0;
        }

        if AXIS & AXIS_Z > 0 && p.z < 0.0 {
            n.z *= -1.0;
        }

        n
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, Uv<Vec3>> for AxialReflectOp<AXIS>
where
    Sdf: Field<Uv<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: Vec3) -> Vec2 {
        let mut pa = p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            pa.z = p.z.abs();
        }

        let mut n = sdf.field(pa);

        if AXIS & AXIS_X > 0 && p.x < 0.0 {
            n.x *= -1.0;
        }

        if AXIS & AXIS_Y > 0 && p.y < 0.0 {
            n.y *= -1.0;
        }

        n
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
