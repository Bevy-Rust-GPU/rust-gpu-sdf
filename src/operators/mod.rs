//! Types that modify a distance field.

pub mod axial_reflect;
pub mod composite;
pub mod displace;
pub mod elongate;
pub mod hollow;
pub mod intersection;
pub mod isosurface;
pub mod reflect;
pub mod repeat;
pub mod rotate;
pub mod scale;
pub mod sided;
pub mod smooth_intersection;
pub mod smooth_subtraction;
pub mod smooth_union;
pub mod stretch;
pub mod subtraction;
pub mod translate;
pub mod twist;
pub mod union;
pub mod conditional;

use crate::signed_distance_field::SignedDistanceField;

/// Modifies the input / output of a [`SignedDistanceField`].
pub trait SignedDistanceOperator<In, Out> {
    fn operator<Sdf>(&self, sdf: &Sdf, p: In) -> Out
    where
        Sdf: SignedDistanceField<In, Out>,
        In: Clone;
}

impl<In, Out> SignedDistanceOperator<In, Out> for () {
    fn operator<Sdf>(&self, sdf: &Sdf, p: In) -> Out
    where
        Sdf: SignedDistanceField<In, Out>,
        In: Clone,
    {
        sdf.evaluate(p)
    }
}

/// Applies a [`SignedDistanceOperator`] to a [`SignedDistanceField`].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, type_fields::Field)]
pub struct Operator<Sdf, Op> {
    pub target: Sdf,
    pub op: Op,
}

impl<Sdf, Op, Dim, Out> SignedDistanceField<Dim, Out> for Operator<Sdf, Op>
where
    Sdf: SignedDistanceField<Dim, Out>,
    Op: SignedDistanceOperator<Dim, Out>,
    Dim: Clone,
{
    fn evaluate(&self, p: Dim) -> Out {
        self.op.operator(&self.target, p)
    }
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Point;

    use super::{isosurface::IsosurfaceOp, Operator};

    #[test]
    fn test_operator() {
        Operator::<Point, IsosurfaceOp>::default()
            .with(Operator::target, Point::default())
            .with(Operator::op, IsosurfaceOp::default());
    }
}
