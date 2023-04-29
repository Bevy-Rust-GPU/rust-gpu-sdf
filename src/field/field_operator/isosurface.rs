//! Shift the isosurface of a distance field by a given amount.

use core::ops::Div;

use type_fields::macros::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        items::position::Position, AttrColor, AttrDistance, AttrNormal, AttrTangent, AttrUv,
        Distance, Field, FieldOperator, Operator,
    },
};

/// Shift the isosurface of a distance field by a given amount.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[cfg_attr(feature = "bevy", derive(bevy::reflect::TypeUuid))]
#[cfg_attr(feature = "bevy", uuid = "d588f817-4e15-4b1e-b98c-dc2b0d47f719")]
#[repr(C)]
pub struct IsosurfaceOp {
    pub delta: f32,
}

impl Default for IsosurfaceOp {
    fn default() -> Self {
        IsosurfaceOp { delta: 1.0 }
    }
}

impl<SdfA, Input> FieldOperator<SdfA, AttrDistance<Input>> for IsosurfaceOp
where
    SdfA: Field<AttrDistance<Input>>,
    Input: Clone,
{
    fn operator(&self, sdf_a: &SdfA, input: &Position<Input>) -> Distance {
        sdf_a.field(input) - self.delta
    }
}

impl_passthrough_op_1!(IsosurfaceOp, AttrNormal<Dim>, Dim);
impl_passthrough_op_1!(IsosurfaceOp, AttrTangent<Dim>, Dim);

impl<SdfA, Input> FieldOperator<SdfA, AttrUv<Input>> for IsosurfaceOp
where
    SdfA: crate::prelude::Field<AttrUv<Input>>,
    Input: Clone + Div<f32, Output = Input>,
{
    fn operator(
        &self,
        sdf_a: &SdfA,
        input: &Position<Input>,
    ) -> <AttrUv<Input> as crate::prelude::Attribute>::Output {
        let p = (*input).clone() / self.delta;
        sdf_a.field(&p)
    }
}

impl_passthrough_op_1!(IsosurfaceOp, AttrColor<Dim>, Dim);

/// Add an arbitrary radius to a distance field.
pub type Isosurface<SdfA> = Operator<IsosurfaceOp, SdfA>;

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::{
        prelude::{Isosurface, Point},
        test_op_attrs,
    };

    #[test]
    fn test_isosurface() {
        Isosurface::<Point>::default();
    }

    test_op_attrs!(Isosurface::<Point>);
}
