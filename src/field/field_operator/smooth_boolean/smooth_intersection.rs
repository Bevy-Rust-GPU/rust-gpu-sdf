//! Compute the blended boolean intersection of two distance fields.

use core::ops::{Add, Div, Mul, Sub};

use rust_gpu_bridge::{glam::Vec2, Clamp, Mix, Normalize, Saturate, Splat, Step};
use type_fields::macros::Field;

use crate::prelude::{
    items::position::Position, AttrDistance, AttrNormal, AttrTangent, AttrUv, Distance, Field,
    FieldOperator, Normal, Operator, Tangent, Uv,
};

/// Compute the blended boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SmoothIntersectionOp {
    pub k: f32,
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrDistance<Input>> for SmoothIntersectionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Input>) -> Distance {
        let d1 = *sdf_a.field(p);
        let d2 = *sdf_b.field(p);
        let h = (0.5 - 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h).add(self.k.mul(h).mul(1.0 - h)).into()
    }
}

impl<SdfA, SdfB, Dim> FieldOperator<(SdfA, SdfB), AttrNormal<Dim>> for SmoothIntersectionOp
where
    SdfA: Field<AttrDistance<Dim>>,
    SdfA: Field<AttrNormal<Dim>>,
    SdfB: Field<AttrDistance<Dim>>,
    SdfB: Field<AttrNormal<Dim>>,
    Dim: Clone
        + Sub<Dim, Output = Dim>
        + Div<f32, Output = Dim>
        + Mul<f32, Output = Dim>
        + Mul<Dim, Output = Dim>
        + Add<f32, Output = Dim>
        + Add<Dim, Output = Dim>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Dim>) -> Normal<Dim> {
        let d1 = (*Field::<AttrDistance<Dim>>::field(sdf_a, p)).clone();
        let d2 = (*Field::<AttrDistance<Dim>>::field(sdf_b, p)).clone();

        let h = (d2 - d1).div(self.k).mul(0.5).sub(0.5).saturate();

        let n1 = (*Field::<AttrNormal<Dim>>::field(sdf_a, p)).clone();
        let n2 = (*Field::<AttrNormal<Dim>>::field(sdf_b, p)).clone();

        n2.mix(n1, Dim::splat(h)).normalize().into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrTangent<Input>> for SmoothIntersectionOp
where
    SdfA: Field<AttrDistance<Input>>,
    SdfA: Field<AttrTangent<Input>>,
    SdfB: Field<AttrDistance<Input>>,
    SdfB: Field<AttrTangent<Input>>,
    Input: Clone
        + Sub<Input, Output = Input>
        + Div<f32, Output = Input>
        + Mul<f32, Output = Input>
        + Mul<Input, Output = Input>
        + Add<Input, Output = Input>
        + Add<f32, Output = Input>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Input>) -> Tangent<Input> {
        let d1 = (*Field::<AttrDistance<Input>>::field(sdf_a, p)).clone();
        let d2 = (*Field::<AttrDistance<Input>>::field(sdf_b, p)).clone();

        let h = (d2 - d1).div(self.k).mul(0.5).sub(0.5).saturate();

        let n1 = (*Field::<AttrTangent<Input>>::field(sdf_a, p)).clone();
        let n2 = (*Field::<AttrTangent<Input>>::field(sdf_b, p)).clone();

        n2.mix(n1, Input::splat(h)).normalize().into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), AttrUv<Input>> for SmoothIntersectionOp
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
        + Add<Input, Output = Input>
        + Add<f32, Output = Input>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Position<Input>) -> Uv {
        let d1 = (*Field::<AttrDistance<Input>>::field(sdf_a, p)).clone();
        let d2 = (*Field::<AttrDistance<Input>>::field(sdf_b, p)).clone();

        let h = (d2 - d1).div(self.k).mul(0.5).sub(0.5).saturate();

        let uv1 = *Field::<AttrUv<Input>>::field(sdf_a, p);
        let uv2 = *Field::<AttrUv<Input>>::field(sdf_b, p);

        uv2.mix(uv1, Vec2::splat(h.step(0.5))).into()
    }
}

/// Compute the blended boolean intersection of two distance fields.
pub type SmoothIntersection<SdfA, SdfB> = Operator<SmoothIntersectionOp, (SdfA, SdfB)>;

impl<SdfA, SdfB> SmoothIntersection<SdfA, SdfB> {
    pub fn sdf_a(&mut self) -> &mut SdfA {
        &mut self.target().0
    }

    pub fn sdf_b(&mut self) -> &mut SdfB {
        &mut self.target().1
    }

    pub fn k(&mut self) -> &mut f32 {
        self.op().k()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use type_fields::field::Field;

    use crate::{
        prelude::{Cube, Point, SmoothIntersection, Sphere},
        test_op_attrs,
    };

    #[test]
    fn test_smooth_intersection() {
        SmoothIntersection::<Cube, Sphere>::default().with(SmoothIntersection::k, f32::default());
    }

    test_op_attrs!(SmoothIntersection::<Point, Point>);
}
