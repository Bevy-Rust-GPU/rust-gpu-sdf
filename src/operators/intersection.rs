//! Compute the boolean intersection of two distance fields.

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

use super::{Operator, SignedDistanceOperator};

/// Compute the boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct IntersectionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfB> SignedDistanceOperator<Vec2> for IntersectionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec2>,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Vec2) -> f32
    where
        SdfA: SignedDistanceField<Vec2>,
    {
        sdf.distance(p).max(self.sdf.distance(p))
    }
}

impl<SdfB> SignedDistanceOperator<Vec3> for IntersectionOp<SdfB>
where
    SdfB: SignedDistanceField<Vec3>,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Vec3) -> f32
    where
        SdfA: SignedDistanceField<Vec3>,
    {
        sdf.distance(p).max(self.sdf.distance(p))
    }
}

/// Compute the boolean intersection of two distance fields.
pub type Intersection<SdfA, SdfB> = Operator<SdfA, IntersectionOp<SdfB>>;

#[allow(non_camel_case_types)]
pub type Intersection_Sdf = (crate::operators::Operator_Op, IntersectionOp_Sdf);

impl<SdfA, SdfB> Intersection<SdfA, SdfB> {
    pub const SDF: Intersection_Sdf = (Operator::<(), ()>::OP, IntersectionOp::<()>::SDF);
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Intersection;

    #[test]
    fn test_intersection() {
        Intersection::<Cube, Sphere>::default()
            .with(Intersection::<(), ()>::SDF, Sphere::default());
    }
}
