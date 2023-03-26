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

use crate::prelude::{Attribute, FieldFunction};

/// Modifies the input / output of a [`SignedDistanceField`].
pub trait SignedDistanceOperator<Sdf, In, Attr>
where
    Attr: Attribute,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: In) -> Attr::Type;
}

impl<Sdf, In, Attr> SignedDistanceOperator<Sdf, In, Attr> for ()
where
    Attr: Attribute,
    Sdf: FieldFunction<In, Attr>,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: In) -> Attr::Type {
        sdf.evaluate(attr, p)
    }
}

/// Applies a [`SignedDistanceOperator`] to a [`SignedDistanceField`].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, type_fields::Field)]
#[repr(C)]
pub struct Operator<Op, Sdf> {
    pub target: Sdf,
    pub op: Op,
}

impl<Op, Sdf, Dim, Attr> FieldFunction<Dim, Attr> for Operator<Op, Sdf>
where
    Attr: Attribute,
    Sdf: FieldFunction<Dim, Attr>,
    Op: SignedDistanceOperator<Sdf, Dim, Attr>,
    Dim: Clone,
{
    fn evaluate(&self, attr: Attr, p: Dim) -> Attr::Type {
        self.op.operator(attr, &self.target, p)
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
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
