//! Compute the blended boolean intersection of two distance fields.

use core::ops::{Add, Div, Mul, Sub};

use rust_gpu_bridge::{glam::Vec2, Clamp, Mix, Normalize, Saturate, Splat, Step};
use type_fields::Field;

use crate::prelude::{Distance, Field, FieldOperator, Normal, Operator, Tangent, Uv};

/// Compute the blended boolean intersection of two distance fields.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SmoothIntersectionOp {
    pub k: f32,
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Distance<Input>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Input>>,
    SdfB: Field<Distance<Input>>,
    Input: Clone,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Input) -> f32 {
        let d1 = sdf_a.field(p);
        let d2 = sdf_b.field(p);
        let h = (0.5 - 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        d2.mix(d1, h).add(self.k.mul(h).mul(1.0 - h)).into()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Normal<Input>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Input>>,
    SdfA: Field<Normal<Input>>,
    SdfB: Field<Distance<Input>>,
    SdfB: Field<Normal<Input>>,
    Input: Clone
        + Sub<Input, Output = Input>
        + Div<f32, Output = Input>
        + Mul<f32, Output = Input>
        + Mul<Input, Output = Input>
        + Add<f32, Output = Input>
        + Add<Input, Output = Input>
        + Clamp
        + Mix
        + Saturate
        + Normalize
        + Splat,
{
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Input) -> Input {
        let d1 = Field::<Distance<Input>>::field(sdf_a, p);
        let d2 = Field::<Distance<Input>>::field(sdf_b, p);

        let h = (d2.clone() - d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let n1 = Field::<Normal<Input>>::field(sdf_a, p);
        let n2 = Field::<Normal<Input>>::field(sdf_b, p);

        n2.mix(n1, Input::splat(h.clone())).normalize()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Tangent<Input>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Input>>,
    SdfA: Field<Tangent<Input>>,
    SdfB: Field<Distance<Input>>,
    SdfB: Field<Tangent<Input>>,
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
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Input) -> Input {
        let d1 = Field::<Distance<Input>>::field(sdf_a, p);
        let d2 = Field::<Distance<Input>>::field(sdf_b, p);

        let h = (d2.clone() - d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let n1 = Field::<Tangent<Input>>::field(sdf_a, p);
        let n2 = Field::<Tangent<Input>>::field(sdf_b, p);

        n2.mix(n1, Input::splat(h.clone())).normalize()
    }
}

impl<SdfA, SdfB, Input> FieldOperator<(SdfA, SdfB), Uv<Input>> for SmoothIntersectionOp
where
    SdfA: Field<Distance<Input>>,
    SdfA: Field<Uv<Input>>,
    SdfB: Field<Distance<Input>>,
    SdfB: Field<Uv<Input>>,
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
    fn operator(&self, (sdf_a, sdf_b): &(SdfA, SdfB), p: &Input) -> Vec2 {
        let d1 = Field::<Distance<Input>>::field(sdf_a, p);
        let d2 = Field::<Distance<Input>>::field(sdf_b, p);

        let h = (d2.clone() - d1.clone())
            .div(self.k)
            .mul(0.5)
            .sub(0.5)
            .saturate();

        let uv1 = Field::<Uv<Input>>::field(sdf_a, p);
        let uv2 = Field::<Uv<Input>>::field(sdf_b, p);

        uv2.mix(uv1, Vec2::splat(h.step(0.5)))
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
