//! Reflect a distance field about an arbitrary axis.

use rust_gpu_bridge::prelude::{Vec2, Vec3, Abs};
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

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

impl<const AXIS: usize> SignedDistanceOperator<f32, Distance> for AxialReflectOp<AXIS> {
    fn operator<Sdf>(&self, sdf: &Sdf, mut p: f32) -> Distance
    where
        Sdf: SignedDistanceField<f32, Distance>,
    {
        if AXIS & AXIS_X > 0 {
            p = p.abs();
        }
        
        sdf.evaluate(p)
    }
}

impl<const AXIS: usize> SignedDistanceOperator<Vec2, Distance> for AxialReflectOp<AXIS> {
    fn operator<Sdf>(&self, sdf: &Sdf, mut p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        if AXIS & AXIS_X > 0 {
            p.x = p.x.abs();
        }

        if AXIS & AXIS_Y > 0 {
            p.y = p.y.abs();
        }

        sdf.evaluate(p)
    }
}

impl<const AXIS: usize> SignedDistanceOperator<Vec3, Distance> for AxialReflectOp<AXIS> {
    fn operator<Sdf>(&self, sdf: &Sdf, mut p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
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
