//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::prelude::{Abs, Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, Operator, DistanceFunction, SignedDistanceOperator};

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

impl<const AXIS: usize, Sdf, Out> SignedDistanceOperator<Sdf, f32, Out> for AxialReflectOp<AXIS>
where
    Sdf: DistanceFunction<f32, Out>,
{
    fn operator(&self, sdf: &Sdf, mut p: f32) -> Out {
        if AXIS & AXIS_X > 0 {
            p = p.abs();
        }

        sdf.evaluate(p)
    }
}

impl<const AXIS: usize, Sdf, Out> SignedDistanceOperator<Sdf, Vec2, Out> for AxialReflectOp<AXIS>
where
    Sdf: DistanceFunction<Vec2, Out>,
{
    fn operator(&self, sdf: &Sdf, mut p: Vec2) -> Out {
        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        sdf.evaluate(p)
    }
}

impl<const AXIS: usize, Sdf, Out> SignedDistanceOperator<Sdf, Vec3, Out> for AxialReflectOp<AXIS>
where
    Sdf: DistanceFunction<Vec3, Out>,
{
    fn operator(&self, sdf: &Sdf, mut p: Vec3) -> Out {
        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        if AXIS & AXIS_Z > 0 {
            p.z = p.z.abs();
        }

        sdf.evaluate(p)
    }
}

/// Reflect a distance field about X / Y / Z
pub type AxialReflect<const AXIS: usize, Sdf> = Operator<AxialReflectOp<AXIS>, Sdf>;
