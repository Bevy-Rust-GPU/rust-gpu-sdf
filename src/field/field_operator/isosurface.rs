//! Shift the isosurface of a distance field by a given amount.

use core::ops::Div;

use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{Color, Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv},
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

impl<SdfA, Input> FieldOperator<SdfA, Distance<Input>> for IsosurfaceOp
where
    SdfA: Field<Distance<Input>>,
    Input: Clone,
{
    fn operator(&self, sdf_a: &SdfA, input: &Input) -> f32 {
        sdf_a.field(input) - self.delta
    }
}

impl_passthrough_op_1!(IsosurfaceOp, Normal<Dim>, Dim);
impl_passthrough_op_1!(IsosurfaceOp, Tangent<Dim>, Dim);

impl<SdfA, Input> FieldOperator<SdfA, Uv<Input>> for IsosurfaceOp
where
    SdfA: crate::prelude::Field<Uv<Input>>,
    Input: Clone + Div<f32, Output = Input>,
{
    fn operator(&self, sdf_a: &SdfA, input: &Input) -> <Uv<Input> as crate::prelude::Attribute>::Output {
        let p = input.clone() / self.delta;
        sdf_a.field(&p)
    }
}

impl_passthrough_op_1!(IsosurfaceOp, Color<Dim>, Dim);

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
