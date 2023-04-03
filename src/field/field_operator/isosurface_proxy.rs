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

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Distance<Dim>> for IsosurfaceProxyOp
where
    SdfA: Field<Distance<Dim>>,
    SdfB: Field<Distance<Dim>>,
    Dim: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: Dim) -> f32 {
        let d1 = sdf_a.field(p.clone());
        let d2 = sdf_b.field(p);
        d1 - d2
    }
}

impl_passthrough_op_2!(IsosurfaceProxyOp, Normal<Dim>, 0, SdfA, Dim);
impl_passthrough_op_2!(IsosurfaceProxyOp, Tangent<Dim>, 0, SdfA, Dim);

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), Uv<Dim>> for IsosurfaceProxyOp
where
    SdfA: crate::prelude::Field<Uv<Dim>>,
    SdfB: crate::prelude::Field<Distance<Dim>>,
    Dim: Clone + Div<f32, Output = Dim>,
{
    fn operator(
        &self,
        (sdf_a, sdf_b): &(SdfA, SdfB),
        p: Dim,
    ) -> <Uv<Dim> as crate::prelude::Attribute>::Output {
        let p = p.clone() / sdf_b.field(p);
        sdf_a.field(p)
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
