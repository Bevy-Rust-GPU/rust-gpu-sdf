//! Vector to nearest surface

use core::{marker::PhantomData, ops::Mul};

use rust_gpu_bridge::IsNormalized;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        Attribute, Color, Distance, FieldFunction, FieldOperator, Normal, Operator, Tangent, Uv,
    },
};

/// Support function attribute marker
pub struct SupportFunctionAttr<Dim> {
    pub _phantom: PhantomData<Dim>,
}

impl<Dim> Default for SupportFunctionAttr<Dim> {
    fn default() -> Self {
        SupportFunctionAttr {
            _phantom: Default::default(),
        }
    }
}

impl<Dim> Attribute for SupportFunctionAttr<Dim> {
    type Type = Dim;
}

/// Support function wrapper operator
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportFunctionOp;

impl<Sdf, Dim> FieldOperator<Sdf, Dim, SupportFunctionAttr<Dim>> for SupportFunctionOp
where
    Sdf: FieldFunction<Dim, Distance> + FieldFunction<Dim, Normal<Dim>>,
    Dim: Default + Clone + Mul<f32, Output = Dim> + IsNormalized,
{
    fn operator(
        &self,
        _: SupportFunctionAttr<Dim>,
        sdf: &Sdf,
        p: Dim,
    ) -> <SupportFunctionAttr<Dim> as Attribute>::Type {
        // Calculate normal
        let n = sdf.evaluate(Normal::<Dim>::default(), p.clone());

        // Skip samples where normal is not valid
        // (ex. the center of a sphere)
        if !n.clone().is_normalized() {
            return Dim::default();
        }

        // Calculate distance
        let d = sdf.evaluate(Distance, p);

        // Calculate vector from position to nearest surface
        n * -d
    }
}

impl_passthrough_op_1!(SupportFunctionOp, Distance, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Tangent<Dim>, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Uv, Dim);
impl_passthrough_op_1!(SupportFunctionOp, Color, Dim);

/// Support function wrapper
pub type SupportFunction<Sdf> = Operator<SupportFunctionOp, Sdf>;
