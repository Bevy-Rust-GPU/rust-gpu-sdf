//! Extrude a shape along its axes, preserving exterior geometry.

use core::{
    marker::PhantomData,
    ops::{Neg, Sub},
};

use rust_gpu_bridge::{
    clamp::Clamp,
    prelude::{Vec2, Vec3},
};

use crate::{
    default,
    markers::{Approx, Exact},
    signed_distance_field::SignedDistanceField,
};

use super::{Operator, SignedDistanceOperator};

/// Extrude a shape along its axes, preserving exterior geometry.
#[derive(Debug)]
pub struct ElongateOp<Precision, Dim> {
    pub extent: Dim,
    pub _phantom: PhantomData<Precision>,
}

impl<Precision> Default for ElongateOp<Precision, Vec2>
{
    fn default() -> Self {
        ElongateOp {
            extent: Vec2::ONE,
            _phantom: default(),
        }
    }
}

impl<Precision> Default for ElongateOp<Precision, Vec3>
{
    fn default() -> Self {
        ElongateOp {
            extent: Vec3::ONE,
            _phantom: default(),
        }
    }
}

impl<Precision, Dim> Clone for ElongateOp<Precision, Dim>
where
    Dim: Clone,
{
    fn clone(&self) -> Self {
        ElongateOp {
            extent: self.extent.clone(),
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Precision, Dim> Copy for ElongateOp<Precision, Dim> where Dim: Copy {}

impl<Precision, Dim> PartialEq for ElongateOp<Precision, Dim>
where
    Dim: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.extent.eq(&other.extent)
    }
}

impl<Dim> SignedDistanceOperator<Dim> for ElongateOp<Approx, Dim>
where
    Dim: Neg<Output = Dim> + Sub<Output = Dim> + Clone + Clamp,
{
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> f32
    where
        Sdf: SignedDistanceField<Dim>,
    {
        let q = p.clone() - p.clone().clamp(-self.extent.clone(), self.extent.clone());
        sdf.distance(q)
    }
}

impl SignedDistanceOperator<Vec3> for ElongateOp<Exact, Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = p.abs() - self.extent;
        sdf.distance(q.max(Vec3::ZERO)) + q.x.max(q.y.max(q.z)).min(0.0)
    }
}

impl SignedDistanceOperator<Vec2> for ElongateOp<Exact, Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> f32
    where
        Sdf: SignedDistanceField<Vec2>,
    {
        let q = p.abs() - self.extent;
        sdf.distance(q.max(Vec2::ZERO)) + q.x.max(q.y).min(0.0)
    }
}

/// Extrude a shape along its axes, preserving exterior geometry.
pub type Elongate<Sdf, Precision, Dim> = Operator<Sdf, ElongateOp<Precision, Dim>, Dim>;
