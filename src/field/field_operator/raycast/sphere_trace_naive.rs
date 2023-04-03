use rust_gpu_bridge::{glam::Vec3, Abs};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        Color, Distance, Field, FieldOperator, Normal, Operator, RaycastInput, Raycast,
        RaycastOutput, Tangent, Uv,
    },
};

/// Basic sphere tracer.
///
/// Marches along a ray, sampling the provided SDF at each step to determine
/// a minimum safe distance for the following iteration.
#[derive(Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SphereTraceNaiveOp<const MAX_STEPS: u32> {
    pub epsilon: f32,
}

impl<const MAX_STEPS: u32> Default for SphereTraceNaiveOp<MAX_STEPS> {
    fn default() -> Self {
        SphereTraceNaiveOp { epsilon: 0.0001 }
    }
}

impl<const MAX_STEPS: u32, Sdf> FieldOperator<Sdf, Raycast>
    for SphereTraceNaiveOp<MAX_STEPS>
where
    Sdf: Field<Distance<Vec3>>,
{
    fn operator(
        &self,
        sdf: &Sdf,
        input: RaycastInput,
    ) -> <Raycast as crate::prelude::Attribute>::Output {
        let mut out = RaycastOutput::default();

        let mut t = input.start;

        for step in 0..MAX_STEPS {
            let pos = input.eye + input.dir * t;
            let dist = sdf.field(pos);

            out.march_step(t, dist);

            if dist < 0.0 {
                out.march_hit(step);
                break;
            }

            t += self.epsilon.max(dist.abs());

            if t > input.end {
                out.march_miss(step);
                break;
            }
        }

        out
    }
}

impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Distance<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Normal<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Tangent<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Uv<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Color<Pos>, Pos, const MAX_STEPS: u32);

pub type SphereTraceNaive<const MAX_STEPS: u32, Sdf> = Operator<SphereTraceNaiveOp<MAX_STEPS>, Sdf>;

impl<const MAX_STEPS: u32, Sdf> SphereTraceNaive<MAX_STEPS, Sdf> {
    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }
}
