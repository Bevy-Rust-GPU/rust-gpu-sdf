use rust_gpu_bridge::{glam::Vec3, Abs};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        Color, Distance, Field, FieldOperator, Normal, Operator, RaycastInput,
        RaycastOutput, Tangent, Uv,
    },
};

/// Basic sphere tracer.
///
/// Marches along a ray, sampling the provided SDF at each step to determine
/// a minimum safe distance for the following iteration.
#[derive(Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct SphereTraceNaiveOp<const MAX_STEPS: u32> {
    pub epsilon: f32,
}

impl<const MAX_STEPS: u32> Default for SphereTraceNaiveOp<MAX_STEPS> {
    fn default() -> Self {
        SphereTraceNaiveOp { epsilon: 0.0001 }
    }
}

impl<const MAX_STEPS: u32, Sdf> FieldOperator<Sdf, RaycastInput, RaycastOutput>
    for SphereTraceNaiveOp<MAX_STEPS>
where
    Sdf: Field<Vec3, Distance>,
{
    fn operator(
        &self,
        mut out: RaycastOutput,
        sdf: &Sdf,
        input: RaycastInput,
    ) -> <RaycastOutput as crate::prelude::Attribute>::Type {
        let mut t = input.start;

        for step in 0..MAX_STEPS {
            let pos = input.eye + input.dir * t;
            let dist = sdf.field(Distance, pos);

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

impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Distance, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Normal<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Tangent<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Uv, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceNaiveOp<MAX_STEPS>, Color, Pos, const MAX_STEPS: u32);

pub type SphereTraceNaive<const MAX_STEPS: u32, Sdf> = Operator<SphereTraceNaiveOp<MAX_STEPS>, Sdf>;

impl<const MAX_STEPS: u32, Sdf> SphereTraceNaive<MAX_STEPS, Sdf> {
    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }
}
