use type_fields::Field;

use crate::signed_distance_field::{attributes::Attribute, DistanceFunction};

use super::{Operator, SignedDistanceOperator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Field)]
#[repr(C)]
pub struct ConditionalOp<Op, const CONDITION: bool> {
    conditional_op: Op,
}

impl<Op, Sdf, Dim, Attr, const CONDITION: bool> SignedDistanceOperator<Sdf, Dim, Attr>
    for ConditionalOp<Op, CONDITION>
where
    Attr: Attribute,
    Sdf: DistanceFunction<Dim, Attr>,
    Op: SignedDistanceOperator<Sdf, Dim, Attr>,
    Dim: Clone,
{
    fn operator(&self, attr: Attr, sdf: &Sdf, p: Dim) -> Attr::Type {
        if CONDITION {
            self.conditional_op.operator(attr, sdf, p)
        } else {
            sdf.evaluate(attr, p)
        }
    }
}

pub type Conditional<Op, Sdf, const CONDITION: bool> = Operator<ConditionalOp<Op, CONDITION>, Sdf>;

impl<Op, Sdf, const CONDITION: bool> Conditional<Op, Sdf, CONDITION> {
    pub fn conditional_op(&mut self) -> &mut Op {
        &mut self.op.conditional_op
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::Vec3;
    use type_fields::field::Field;

    use crate::{prelude::StretchDistOp, signed_distance_field::shapes::composite::Sphere};

    use super::Conditional;

    #[test]
    pub fn test_conditional() {
        Conditional::<StretchDistOp<Vec3>, Sphere, true>::default()
            .with(Conditional::conditional_op, StretchDistOp::default());
    }
}
