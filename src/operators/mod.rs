//! Types that modify a distance field.

pub mod axial_reflect;
pub mod compose;
pub mod composite;
pub mod conditional;
pub mod displace;
pub mod elongate;
pub mod hollow;
pub mod intersection;
pub mod isosurface;
pub mod normalize;
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

use crate::signed_distance_field::DistanceFunction;

/// Modifies the input / output of a [`SignedDistanceField`].
pub trait SignedDistanceOperator<Sdf, In, Out> {
    fn operator(&self, sdf: &Sdf, p: In) -> Out;
}

impl<Sdf, In, Out> SignedDistanceOperator<Sdf, In, Out> for ()
where
    Sdf: DistanceFunction<In, Out>,
{
    fn operator(&self, sdf: &Sdf, p: In) -> Out {
        sdf.evaluate(p)
    }
}

/// Applies a [`SignedDistanceOperator`] to a [`SignedDistanceField`].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, type_fields::Field)]
#[repr(C)]
pub struct Operator<Op, Sdf> {
    pub target: Sdf,
    pub op: Op,
}

impl<Op, Sdf, Dim, Out> DistanceFunction<Dim, Out> for Operator<Op, Sdf>
where
    Sdf: DistanceFunction<Dim, Out>,
    Op: SignedDistanceOperator<Sdf, Dim, Out>,
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
        Operator::<IsosurfaceOp, Point>::default()
            .with(Operator::target, Point::default())
            .with(Operator::op, IsosurfaceOp::default());
    }
}
