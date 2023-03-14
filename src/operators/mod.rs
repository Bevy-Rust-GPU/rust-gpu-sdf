//! Types that modify a distance field.

pub mod axial_reflect;
pub mod composite;
pub mod conditional;
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

pub mod compose {
    use rust_gpu_bridge::prelude::{Vec2, Vec3, Vec3Swizzles};
    use type_fields::Field;

    use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

    use super::{Operator, SignedDistanceOperator};

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
    pub struct ComposeOp<SdfA, SdfB> {
        pub sdf_a: SdfA,
        pub sdf_b: SdfB,
    }

    impl<SdfA, SdfB> SignedDistanceOperator<Vec2, Distance> for ComposeOp<SdfA, SdfB>
    where
        SdfA: SignedDistanceField<f32, Distance>,
        SdfB: SignedDistanceField<f32, Distance>,
    {
        fn operator<Sdf>(&self, sdf: &Sdf, mut p: Vec2) -> Distance
        where
            Sdf: SignedDistanceField<Vec2, Distance>,
        {
            p.x += *self.sdf_a.evaluate(p.x);
            p.y += *self.sdf_b.evaluate(p.y);
            sdf.evaluate(p)
        }
    }

    impl<SdfA, SdfB> SignedDistanceOperator<Vec3, Distance> for ComposeOp<SdfA, SdfB>
    where
        SdfA: SignedDistanceField<Vec2, Distance>,
        SdfB: SignedDistanceField<f32, Distance>,
    {
        fn operator<Sdf>(&self, sdf: &Sdf, mut p: Vec3) -> Distance
        where
            Sdf: SignedDistanceField<Vec3, Distance>,
        {
            p.x += *self.sdf_a.evaluate(p.xy());
            p.y += *self.sdf_b.evaluate(p.z);
            sdf.evaluate(p)
        }
    }

    pub type Compose<Sdf, SdfA, SdfB> = Operator<ComposeOp<SdfA, SdfB>, Sdf>;

    impl<Sdf, SdfA, SdfB> Compose<Sdf, SdfA, SdfB> {
        pub fn sdf_a(&mut self) -> &mut SdfA {
            &mut self.op.sdf_a
        }

        pub fn sdf_b(&mut self) -> &mut SdfB {
            &mut self.op.sdf_b
        }
    }
}

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
pub struct Operator<Op, Sdf> {
    pub target: Sdf,
    pub op: Op,
}

impl<Op, Sdf, Dim, Out> SignedDistanceField<Dim, Out> for Operator<Op, Sdf>
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
        Operator::<IsosurfaceOp, Point>::default()
            .with(Operator::target, Point::default())
            .with(Operator::op, IsosurfaceOp::default());
    }
}
