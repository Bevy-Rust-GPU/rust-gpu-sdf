//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::macros::Field;

use crate::prelude::{
    items::position::Position, Attribute, AttrDistance, Field, FieldOperator, AttrNormal, Operator, AttrUv, Distance, Normal, Uv,
};

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
    Attr: Attribute<Input = Position<f32>>,
    Sdf: Field<Attr>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<f32>) -> Attr::Output {
        let mut input = **input;

        if AXIS & AXIS_X > 0 {
            input = input.abs();
        }

        sdf.field(&input.into())
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, AttrDistance<Vec2>> for AxialReflectOp<AXIS>
where
    Sdf: Field<AttrDistance<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, input: &Position<Vec2>) -> Distance {
        let mut input = **input;

        if AXIS & AXIS_X > 0 {
            input.x = input.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            input.y = input.y.abs();
        }

        sdf.field(&input.into())
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, AttrNormal<Vec2>> for AxialReflectOp<AXIS>
where
    Sdf: Field<AttrNormal<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Normal<Vec2> {
        let mut pa = **p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        let mut n = sdf.field(&pa.into());

        if AXIS & AXIS_X > 0 && p.x < 0.0 {
            n.x *= -1.0;
        }

        if AXIS & AXIS_Y > 0 && p.y < 0.0 {
            n.y *= -1.0;
        }

        n
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, AttrUv<Vec2>> for AxialReflectOp<AXIS>
where
    Sdf: Field<AttrUv<Vec2>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec2>) -> Uv {
        let mut pa = *p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        let mut n = sdf.field(&pa);

        if AXIS & AXIS_X > 0 && p.x < 0.0 {
            n.x *= -1.0;
        }

        if AXIS & AXIS_Y > 0 && p.y < 0.0 {
            n.y *= -1.0;
        }

        n
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, AttrDistance<Vec3>> for AxialReflectOp<AXIS>
where
    Sdf: Field<AttrDistance<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> Distance {
        let mut p = *p;

        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            p.z = p.z.abs();
        }

        sdf.field(&p)
    }
}

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, AttrNormal<Vec3>> for AxialReflectOp<AXIS>
where
    Sdf: Field<AttrNormal<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> Normal<Vec3> {
        let mut pa = *p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            pa.z = p.z.abs();
        }

        let mut n = sdf.field(&pa);

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

impl<const AXIS: usize, Sdf> FieldOperator<Sdf, AttrUv<Vec3>> for AxialReflectOp<AXIS>
where
    Sdf: Field<AttrUv<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Vec3>) -> Uv {
        let mut pa = *p;

        if AXIS & AXIS_X > 0 {
            pa.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            pa.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            pa.z = p.z.abs();
        }

        let mut n = sdf.field(&pa);

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
