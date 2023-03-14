use type_fields::Field;

use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

use super::{Operator, SignedDistanceOperator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Field)]
pub struct ConditionalOp<Op, const CONDITION: bool> {
    conditional_op: Op,
}

impl<Op, Dim, const CONDITION: bool> SignedDistanceOperator<Dim, Distance>
    for ConditionalOp<Op, CONDITION>
where
    Op: SignedDistanceOperator<Dim, Distance>,
{
    fn operator<Sdf>(&self, sdf: &Sdf, p: Dim) -> Distance
    where
        Sdf: SignedDistanceField<Dim, Distance>,
        Dim: Clone,
    {
        if CONDITION {
            self.conditional_op.operator(sdf, p)
        } else {
            sdf.evaluate(p)
        }
    }
}

pub type Conditional<Sdf, Op, const CONDITION: bool> =
    Operator<Sdf, ConditionalOp<Op, CONDITION>>;

impl<Sdf, Op, const CONDITION: bool> Conditional<Sdf, Op, CONDITION> {
    pub fn conditional_op(&mut self) -> &mut Op {
        &mut self.op.conditional_op
    }
}

#[cfg(test)]
pub mod test {
    use rust_gpu_bridge::prelude::Vec3;
    use type_fields::field::Field;

    use crate::{prelude::StretchDistOp, signed_distance_field::shapes::composite::Sphere};

    use super::Conditional;

    #[test]
    pub fn test_conditional() {
        Conditional::<Sphere, StretchDistOp<Vec3>, true>::default()
            .with(Conditional::conditional_op, StretchDistOp::default());
    }
}

