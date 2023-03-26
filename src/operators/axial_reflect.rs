//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::{
    prelude::{DistanceFunction, Operator, SignedDistanceOperator},
    signed_distance_field::attributes::Attribute,
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
#[repr(C)]
pub struct AxialReflectOp<const AXIS: usize>;

impl<const AXIS: usize, Sdf, Attr> SignedDistanceOperator<Sdf, f32, Attr> for AxialReflectOp<AXIS>
where
    Attr: Attribute,
    Sdf: DistanceFunction<f32, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, mut p: f32) -> Attr::Type {
        if AXIS & AXIS_X > 0 {
            p = p.abs();
        }

        sdf.evaluate(attr, p)
    }
}

impl<const AXIS: usize, Sdf, Attr> SignedDistanceOperator<Sdf, Vec2, Attr> for AxialReflectOp<AXIS>
where
    Attr: Attribute,
    Sdf: DistanceFunction<Vec2, Attr>,
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

impl<const AXIS: usize, Sdf, Attr> SignedDistanceOperator<Sdf, Vec3, Attr> for AxialReflectOp<AXIS>
where
    Attr: Attribute,
    Sdf: DistanceFunction<Vec3, Attr>,
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
