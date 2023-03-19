//! Compute the boolean union of two distance fields.

use type_fields::Field;

use crate::{
    prelude::{Distance, Operator, DistanceFunction, SignedDistanceOperator},
    signed_distance_field::attributes::{normal::Normal, uv::Uv},
};

/// Compute the boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
#[repr(C)]
pub struct UnionOp<Sdf> {
    pub sdf: Sdf,
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Distance> for UnionOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Distance>,
    SdfB: DistanceFunction<Dim, Distance>,
    Dim: Clone,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Distance
    {
        sdf.evaluate(p.clone()).min(*self.sdf.evaluate(p)).into()
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Normal<Dim>> for UnionOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Distance>,
    SdfA: DistanceFunction<Dim, Normal<Dim>>,
    SdfB: DistanceFunction<Dim, Distance>,
    SdfB: DistanceFunction<Dim, Normal<Dim>>,
    Dim: Clone,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Normal<Dim> {
        let dist_a: Distance = sdf.evaluate(p.clone());
        let dist_b: Distance = self.sdf.evaluate(p.clone());

        let n: Normal<Dim> = if *dist_a < *dist_b {
            sdf.evaluate(p)
        } else {
            self.sdf.evaluate(p)
        };

        n.into()
    }
}

impl<SdfA, SdfB, Dim> SignedDistanceOperator<SdfA, Dim, Uv> for UnionOp<SdfB>
where
    SdfA: DistanceFunction<Dim, Distance>,
    SdfA: DistanceFunction<Dim, Uv>,
    SdfB: DistanceFunction<Dim, Distance>,
    SdfB: DistanceFunction<Dim, Uv>,
    Dim: Clone,
{
    fn operator(&self, sdf: &SdfA, p: Dim) -> Uv {
        let dist_a: Distance = sdf.evaluate(p.clone());
        let dist_b: Distance = self.sdf.evaluate(p.clone());

        let uv: Uv = if *dist_a < *dist_b {
            sdf.evaluate(p)
        } else {
            self.sdf.evaluate(p)
        };

        uv.into()
    }
}

/// Compute the boolean union of two distance fields.
pub type Union<SdfA, SdfB> = Operator<UnionOp<SdfB>, SdfA>;

impl<SdfA, SdfB> Union<SdfA, SdfB> {
    pub fn sdf(&mut self) -> &mut SdfB {
        &mut self.op.sdf
    }
}

#[cfg(test)]
pub mod test {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::{Cube, Sphere};

    use super::Union;

    #[test]
    fn test_union() {
        Union::<Sphere, Cube>::default().with(Union::sdf, Cube::default());
    }
}
