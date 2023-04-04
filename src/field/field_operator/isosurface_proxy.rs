//! Shift the isosurface of a distance field by a given amount.

use core::ops::Div;

use type_fields::Field;

use crate::{
    impl_passthrough_op_2,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv,
        DisplaceProxyOp, Distance, Field, FieldOperator, Operator,
    },
};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct IsosurfaceProxyOp;

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrDistance<Input>> for IsosurfaceProxyOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Input>) -> Distance {
        let d1 = sdf_a.field(p);
        let d2 = sdf_b.field(p);
        d1 - *d2
    }
}

impl_passthrough_op_2!(IsosurfaceProxyOp, AttrNormal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(IsosurfaceProxyOp, AttrTangent<Dim>, 0, SdfA, Dim);

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrUv<Input>> for IsosurfaceProxyOp
where
    SdfA: crate::prelude::Field<AttrUv<Input>>,
    SdfB: crate::prelude::Field<AttrDistance<Input>>,
    Input: Clone + Div<f32, Output = Input>,
{
    fn operator(
        &self,
        (sdf_a, sdf_b): &(SdfA, SdfB),
        input: &Position<Input>,
    ) -> <AttrUv<Input> as crate::prelude::Attribute>::Output {
        let p = (*input).clone() / *sdf_b.field(input);
        sdf_a.field(&p)
    }
}

impl_passthrough_op_2!(IsosurfaceProxyOp, AttrColor<Dim>, 0, SdfA, Dim);

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
