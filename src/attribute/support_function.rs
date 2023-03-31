//! Vector to nearest surface

use core::ops::Mul;

use rust_gpu_bridge::IsNormalized;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        Attribute, Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv,
    },
};

/// Support function attribute marker
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
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
    type Type = Self;
}

/// Support function wrapper operator
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportFunctionOp;

impl<Sdf, Dim> FieldOperator<Sdf, Dim, Support<Dim>> for SupportFunctionOp
where
    Sdf: Field<Dim, Distance> + Field<Dim, Normal<Dim>>,
    Dim: Default + Clone + Mul<f32, Output = Dim> + IsNormalized,
{
    fn operator(
        &self,
        mut out: Support<Dim>,
        sdf: &Sdf,
        p: Dim,
    ) -> <Support<Dim> as Attribute>::Type {
        // Calculate normal
        let n = sdf.field(Normal::<Dim>::default(), p.clone());

        // Skip samples where normal is not valid
        // (ex. the center of a sphere)
        if !n.clone().is_normalized() {
            return out;
        }

        // Calculate distance
        let d = sdf.field(Distance, p);

        // Write into output
        out.normal = n;
        out.distance = d;

        out
    }
}

impl_passthrough_op_1!(SupportFunctionOp, Distance, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Uv, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Color, Dim);

/// Support function wrapper
pub type SupportFunction<Sdf> = Operator<SupportFunctionOp, Sdf>;
