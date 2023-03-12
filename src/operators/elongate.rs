//! Extrude a shape along its axes, preserving exterior geometry.

use core::ops::Add;

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Extrude a shape along its axes, preserving exterior geometry.
#[derive(Debug, Field)]
pub struct ElongateOp<Dim> {
    pub extent: Dim,
}

impl Default for ElongateOp<Vec2> {
    fn default() -> Self {
        ElongateOp { extent: Vec2::ONE }
    }
}

impl Default for ElongateOp<Vec3> {
    fn default() -> Self {
        ElongateOp { extent: Vec3::ONE }
    }
}

impl SignedDistanceOperator<Vec2, Distance> for ElongateOp<Vec2> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec2) -> Distance
    where
        Sdf: SignedDistanceField<Vec2, Distance>,
    {
        let q = p.abs() - self.extent;
        sdf.evaluate(q.max(Vec2::ZERO))
            .add(q.x.max(q.y).min(0.0))
            .into()
    }
}

impl SignedDistanceOperator<Vec3, Distance> for ElongateOp<Vec3> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: Vec3) -> Distance
    where
        Sdf: SignedDistanceField<Vec3, Distance>,
    {
        let q = p.abs() - self.extent;
        sdf.evaluate(q.max(Vec3::ZERO))
            .add(q.x.max(q.y.max(q.z)).min(0.0))
            .into()
    }
}

/// Extrude a shape along its axes, preserving exterior geometry.
pub type Elongate<Sdf, Dim> = Operator<Sdf, ElongateOp<Dim>>;

#[allow(non_camel_case_types)]
pub type Elongate_Extent = (crate::operators::Operator_Op, ElongateOp_Extent);

impl<Sdf, Dim> Elongate<Sdf, Dim> {
    pub const EXTENT: Elongate_Extent = (Operator::<(), ()>::OP, ElongateOp::<()>::EXTENT);
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Point;

    use super::Elongate;

    #[test]
    fn test_elongate() {
        Elongate::<Point, _>::default().with(Elongate::<(), ()>::EXTENT, Vec3::default());
    }
}
