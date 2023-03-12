//! Compute the boolean intersection of two distance fields.

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Compute the boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct IntersectionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfB> SignedDistanceOperator<Vec2, Distance> for IntersectionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec2, Distance>,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Vec2) -> Distance
    where
        SdfA: SignedDistanceField<Vec2, Distance>,
    {
        sdf.evaluate(p).max(*self.sdf.evaluate(p)).into()
    }
}

impl<SdfB> SignedDistanceOperator<Vec3, Distance> for IntersectionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec3, Distance>,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Vec3) -> Distance
    where
        SdfA: SignedDistanceField<Vec3, Distance>,
    {
        sdf.evaluate(p).max(*self.sdf.evaluate(p)).into()
    }
}

/// Compute the boolean intersection of two distance fields.
pub type Intersection<SdfA, SdfB> = Operator<SdfA, IntersectionOp<SdfB>>;

impl<SdfA, SdfB> Intersection<SdfA, SdfB> {
    pub fn sdf(&mut self) -> &mut SdfB {
        &mut self.op.sdf
    }
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Intersection;

    #[test]
    fn test_intersection() {
        Intersection::<Cube, Sphere>::default()
            .with(Intersection::sdf, Sphere::default());
    }
}
