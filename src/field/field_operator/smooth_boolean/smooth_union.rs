//! Compute the blended boolean union of two distance fields.

use core::ops::{Add, Div, Mul, Sub};

use rust_gpu_bridge::{glam::Vec2, Clamp, Mix, Normalize, Saturate, Splat};
use type_fields::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrTangent, AttrUv, Distance, Field,
    FieldOperator, Normal, Operator, Tangent, Uv,
};

/// Compute the blended boolean union of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SmoothUnionOp {
    pub k: f32,
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrDistance<Input>> for SmoothUnionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Distance {
        let d1 = *sdf_a.field(input);
        let d2 = *sdf_b.field(input);
        let h = (0.5 + 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h).sub(self.k * h * (1.0 - h)).into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrNormal<Input>> for SmoothUnionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfA: Field<AttrNormal<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    SdfB: Field<AttrNormal<Input>>,
    Input: Clone
        + Sub<Input, Output = Input>
        + Div<f32, Output = Input>
        + Mul<f32, Output = Input>
        + Mul<Input, Output = Input>
        + Add<f32, Output = Input>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Normal<Input> {
        let d1 = *Field::<AttrDistance<Input>>::field(sdf_a, input);
        let d2 = *Field::<AttrDistance<Input>>::field(sdf_b, input);

        let h = ((d2.clone() - d1.clone()).div(self.k).mul(0.5).add(0.5)).saturate();

        let n1 = (*Field::<AttrNormal<Input>>::field(sdf_a, input)).clone();
        let n2 = (*Field::<AttrNormal<Input>>::field(sdf_b, input)).clone();

        n2.mix(n1, Input::splat(h)).normalize().into()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), AttrTangent<Dim>> for SmoothUnionOp
where
    SdfA: Field<AttrDistance<Dim>>,
    SdfA: Field<AttrTangent<Dim>>,
    SdfB: Field<AttrDistance<Dim>>,
    SdfB: Field<AttrTangent<Dim>>,
    Dim: Clone
        + Sub<Dim, Output = Dim>
        + Div<f32, Output = Dim>
        + Mul<f32, Output = Dim>
        + Mul<Dim, Output = Dim>
        + Add<f32, Output = Dim>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Dim>) -> Tangent<Dim> {
        let d1 = *Field::<AttrDistance<Dim>>::field(sdf_a, p);
        let d2 = *Field::<AttrDistance<Dim>>::field(sdf_b, p);
        let h = ((d2.clone() - d1.clone()).div(self.k).mul(0.5).add(0.5)).saturate();

        let t1 = (*Field::<AttrTangent<Dim>>::field(sdf_a, p)).clone();
        let t2 = (*Field::<AttrTangent<Dim>>::field(sdf_b, p)).clone();

        t2.mix(t1, Dim::splat(h)).normalize().into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrUv<Input>> for SmoothUnionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfA: Field<AttrUv<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    SdfB: Field<AttrUv<Input>>,
    Input: Clone
        + Sub<Input, Output = Input>
        + Div<f32, Output = Input>
        + Mul<f32, Output = Input>
        + Mul<Input, Output = Input>
        + Add<f32, Output = Input>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), input: &Position<Input>) -> Uv {
        let d1 = *Field::<AttrDistance<Input>>::field(sdf_a, input);
        let d2 = *Field::<AttrDistance<Input>>::field(sdf_b, input);

        let h = ((d2.clone() - d1.clone()).div(self.k).mul(0.5).add(0.5)).saturate();

        let uv1 = Field::<AttrUv<Input>>::field(sdf_a, input);
        let uv2 = Field::<AttrUv<Input>>::field(sdf_b, input);

        if h > 0.5 {
            uv1
        } else {
            uv2
        }
    }
}

/// Compute the blended boolean union of two distance fields.
pub type SmoothUnion<SdfA, SdfB> = Operator<SmoothUnionOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> SmoothUnion<SdfA, SdfB> {
    pub fn sdf_a(&mut self) -> &mut SdfA {
        &mut self.target().0
    }

    pub fn sdf_b(&mut self) -> &mut SdfB {
        &mut self.target().1
    }

    pub fn k(&mut self) -> &mut f32 {
        &mut self.op.k
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, SmoothUnion, Sphere},
        test_op_attrs,
    };

    #[test]
    fn test_smooth_union() {
        SmoothUnion::<Cube, Sphere>::default().with(SmoothUnion::k, f32::default());
    }

    test_op_attrs!(SmoothUnion::<Point, Point>);
}
