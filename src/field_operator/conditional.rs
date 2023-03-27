use type_fields::Field;

use crate::prelude::{Attribute, FieldFunction};

use super::{FieldOperator, Operator};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Field)]
#[repr(C)]
pub struct ConditionalOp<Op, const CONDITION: bool> {
    conditional_op: Op,
}

impl<Op, Sdf, Dim, Attr, const CONDITION: bool> FieldOperator<Sdf, Dim, Attr>
    for ConditionalOp<Op, CONDITION>
where
    Attr: Attribute,
    Sdf: FieldFunction<Dim, Attr>,
    Op: FieldOperator<Sdf, Dim, Attr>,
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

    use crate::{
        prelude::{IsosurfaceOp, Point, Sphere, StretchDistOp},
        test_op_attrs,
    };

    use super::Conditional;

    #[test]
    pub fn test_conditional() {
        Conditional::<StretchDistOp<Vec3>, Sphere, true>::default()
            .with(Conditional::conditional_op, StretchDistOp::default());
    }

    test_op_attrs!(Conditional::<IsosurfaceOp, Point, false>);
}
