//! Displace the output of a distance field using the output of another distance field.

use core::ops::Add;

use type_fields::Field;

use crate::prelude::{Distance, Operator, SignedDistanceField, SignedDistanceOperator};

/// Displace the output of a distance field using the output of another distance field.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct DisplaceOp<Sdf> {
    pub displace: Sdf,
}

impl<SdfB, Dim> SignedDistanceOperator<Dim, Distance> for DisplaceOp<SdfB>
where
    SdfB: SignedDistanceField<Dim, Distance>,
{
    fn operator<SdfA>(&self, sdf: &SdfA, p: Dim) -> Distance
    where
        SdfA: SignedDistanceField<Dim, Distance>,
        Dim: Clone,
    {
        (*sdf.evaluate(p.clone()))
            .add(*self.displace.evaluate(p))
            .into()
    }
}

/// Displace the output of a distance field using the output of another distance field.
pub type Displace<SdfA, SdfB> = Operator<SdfA, DisplaceOp<SdfB>>;
impl<SdfA, SdfB> Displace<SdfA, SdfB> {
    pub fn displace(&mut self) -> &mut SdfB {
        &mut self.op.displace
    }
}

#[cfg(test)]
pub mod tests {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Displace;

    #[test]
    fn test_displace() {
        Displace::<Cube, Sphere>::default().with(Displace::displace, Sphere::default());
    }
}
