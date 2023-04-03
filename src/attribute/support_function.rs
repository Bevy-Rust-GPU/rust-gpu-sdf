//! Vector to nearest surface

use core::ops::Mul;

use rust_gpu_bridge::IsNormalized;

use crate::{
    impl_passthrough_op_1,
    prelude::{Attribute, Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
};

/// Support function attribute marker
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Support<Dim> {
    pub normal: Dim,
    pub distance: f32,
}

impl<Dim> Support<Dim> {
    pub fn support_vector(&self) -> Dim
    where
        Dim: Clone + Mul<f32, Output = Dim>,
    {
        self.normal.clone() * -self.distance
    }
}

impl<Dim> Attribute for Support<Dim>
where
    Dim: Default,
{
    type Input = Dim;
    type Output = Self;
}

/// Support function wrapper operator
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SupportFunctionOp;

impl<Sdf, Input> FieldOperator<Sdf, Support<Input>> for SupportFunctionOp
where
    Sdf: Field<Distance<Input>> + Field<Normal<Input>>,
    Input: Default + Clone + Mul<f32, Output = Input> + IsNormalized,
{
    fn operator(&self, sdf: &Sdf, p: &Input) -> <Support<Input> as Attribute>::Output {
        let mut out = Support::default();

        // Calculate normal
        let n = Field::<Normal<Input>>::field(sdf, p);

        // Skip samples where normal is not valid
        // (ex. the center of a sphere)
        if !n.clone().is_normalized() {
            return out;
        }

        // Calculate distance
        let d = Field::<Distance<Input>>::field(sdf, p);

        // Write into output
        out.normal = n;
        out.distance = d;

        out
    }
}

impl_passthrough_op_1!(SupportFunctionOp, Distance<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Uv<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Color<Dim>, Dim);

/// Support function wrapper
pub type SupportFunction<Sdf> = Operator<SupportFunctionOp, Sdf>;
