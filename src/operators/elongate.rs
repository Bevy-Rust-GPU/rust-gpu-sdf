//! Extrude a shape along its axes, preserving exterior geometry.

use core::marker::PhantomData;

use rust_gpu_bridge::prelude::Vec3;

use crate::{
    default,
    markers::{Approx, Exact}, signed_distance_field::SignedDistanceField,
};

use super::{Operator, SignedDistanceOperator};

/// Extrude a shape along its axes, preserving exterior geometry.
#[derive(Debug)]
pub struct ElongateOp<Precision> {
    pub extent: Vec3,
    pub _phantom: PhantomData<Precision>,
}

impl<Precision> Default for ElongateOp<Precision> {
    fn default() -> Self {
        ElongateOp {
            extent: Vec3::ONE,
            _phantom: default(),
        }
    }
}

impl<Precision> Clone for ElongateOp<Precision> {
    fn clone(&self) -> Self {
        ElongateOp {
            extent: self.extent.clone(),
            _phantom: self._phantom.clone(),
        }
    }
}

impl<Precision> Copy for ElongateOp<Precision> {}

impl<Precision> PartialEq for ElongateOp<Precision> {
    fn eq(&self, other: &Self) -> bool {
        self.extent.eq(&other.extent)
    }
}

impl SignedDistanceOperator<Vec3> for ElongateOp<Approx> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = p - p.clamp(-self.extent, self.extent);
        sdf.distance(q)
    }
}

impl SignedDistanceOperator<Vec3> for ElongateOp<Exact> {
    fn operator<Sdf>(&self, sdf: Sdf, p: Vec3) -> f32
    where
        Sdf: SignedDistanceField<Vec3>,
    {
        let q = p.abs() - self.extent;
        sdf.distance(q.max(Vec3::ZERO)) + q.x.max(q.y.max(q.z)).min(0.0)
    }
}

/// Extrude a shape along its axes, preserving exterior geometry.
pub type Elongate<Sdf, Precision> = Operator<Sdf, ElongateOp<Precision>, Vec3>;

