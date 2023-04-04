//! Vector to nearest surface

use core::{marker::PhantomData, ops::Mul};

use rust_gpu_bridge::IsNormalized;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv,
        Attribute, Distance, Field, FieldOperator, Operator,
    },
};

/// Support function attribute marker
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Support<Dim> {
    pub normal: Dim,
    pub distance: Distance,
}

impl<Dim> Support<Dim> {
    pub fn support_vector(&self) -> Dim
    where
        Dim: Clone + Mul<f32, Output = Dim>,
    {
        self.normal.clone() * -*self.distance
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttrSupport<Dim> {
    _phantom: PhantomData<Dim>,
}

impl<Dim> Attribute for AttrSupport<Dim>
where
    Dim: Default,
{
    type Input = Position<Dim>;
    type Output = Support<Dim>;
}

/// Support function wrapper operator
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SupportFunctionOp;

impl<Sdf, Dim> FieldOperator<Sdf, AttrSupport<Dim>> for SupportFunctionOp
where
    Sdf: Field<AttrDistance<Dim>> + Field<AttrNormal<Dim>>,
    Dim: Default + Clone + Mul<f32, Output = Dim> + IsNormalized,
{
    fn operator(&self, sdf: &Sdf, p: &Position<Dim>) -> <AttrSupport<Dim> as Attribute>::Output {
        let mut out = Support::default();

        // Calculate normal
        let n = (*Field::<AttrNormal<Dim>>::field(sdf, p)).clone();

        // Skip samples where normal is not valid
        // (ex. the center of a sphere)
        if !n.clone().is_normalized() {
            return out;
        }

        // Calculate distance
        let d = Field::<AttrDistance<Dim>>::field(sdf, p);

        // Write into output
        out.normal = n;
        out.distance = d;

        out
    }
}

impl_passthrough_op_1!(SupportFunctionOp, AttrDistance<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, AttrTangent<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, AttrUv<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, AttrColor<Dim>, Dim);

/// Support function wrapper
pub type SupportFunction<Sdf> = Operator<SupportFunctionOp, Sdf>;
