//! Shift the isosurface of a distance field by a given amount.

use core::ops::Div;

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{
        Color, DisplaceProxyOp, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv,
    },
};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct IsosurfaceProxyOp;

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Distance<Input>> for IsosurfaceProxyOp
where
    SdfA: Field<Distance<Input>>,
    SdfB: Field<Distance<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Input) -> f32 {
        let d1 = sdf_a.field(p);
        let d2 = sdf_b.field(p);
        d1 - d2
    }
}

impl_passthrough_op_2!(IsosurfaceProxyOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(IsosurfaceProxyOp, Tangent<Dim>, 0, SdfA, Dim);

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Uv<Input>> for IsosurfaceProxyOp
where
    SdfA: crate::prelude::Field<Uv<Input>>,
    SdfB: crate::prelude::Field<Distance<Input>>,
    Input: Clone + Div<f32, Output = Input>,
{
    fn operator(
        &self,
        (sdf_a, sdf_b): &(SdfA, SdfB),
        input: &Input,
    ) -> <Uv<Input> as crate::prelude::Attribute>::Output {
        let p = input.clone() / sdf_b.field(input);
        sdf_a.field(&p)
    }
}

impl_passthrough_op_2!(IsosurfaceProxyOp, Color<Dim>, 0, SdfA, Dim);

/// Add an arbitrary radius to a distance field.
pub type IsosurfaceProxy<SdfA, SdfB> = Operator<DisplaceProxyOp, (SdfA, SdfB)>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{IsosurfaceProxy, Point},
        test_op_attrs,
    };

    #[test]
    fn test_isosurface() {
        IsosurfaceProxy::<Point, f32>::default();
    }

    test_op_attrs!(IsosurfaceProxy::<Point, f32>);
}
