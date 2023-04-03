use rust_gpu_bridge::{glam::Vec3, Abs};
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        Color, Distance, Field, FieldOperator, Normal, Operator, Raycast, RaycastOutput, Tangent,
        Uv,
    },
};

use super::RaycastInput;

/// Sphere tracer that operates with respect to a precomputed Lipschitz bound.
///
/// Costs 1 extra divide per step compared to [`SphereTraceNaive`],
/// but results in overall faster intersections.
///
/// Note: The precomputed lipschitz bound `k` must be correct in respect to the
/// provided SDF for accurate behaviour; incorrect values will result in visual artifacting.
#[derive(Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SphereTraceLipschitzOp<const MAX_STEPS: u32> {
    pub epsilon: f32,
    pub frac_1_k: f32,
}

impl<const MAX_STEPS: u32> Default for SphereTraceLipschitzOp<MAX_STEPS> {
    fn default() -> Self {
        SphereTraceLipschitzOp {
            epsilon: 0.0001,
            frac_1_k: 1.0 / (SphereTraceLipschitzOp::<MAX_STEPS>::falloff_k(1.0, 3.0) * 3.0),
        }
    }
}

impl<const MAX_STEPS: u32> SphereTraceLipschitzOp<MAX_STEPS> {
    // Computes the global lipschitz bound of the falloff function
    // e: energy
    // R: radius
    fn falloff_k(e: f32, r: f32) -> f32 {
        1.72 * e.abs() / r
    }
}

impl<const MAX_STEPS: u32, Sdf> FieldOperator<Sdf, Raycast> for SphereTraceLipschitzOp<MAX_STEPS>
where
    Sdf: Field<Distance<Vec3>>,
{
    fn operator(&self, sdf: &Sdf, input: &RaycastInput) -> RaycastOutput {
        let mut out = RaycastOutput::default();

        let mut t = input.start;
        for i in 0..MAX_STEPS {
            let pos = input.eye + input.dir * t;
            let dist = sdf.field(&pos);

            out.march_step(t, dist);

            if dist < 0.0 {
                out.march_hit(i);
                break;
            }

            t += self.epsilon.max(dist.abs() * self.frac_1_k);

            if t > input.end {
                out.march_miss(i);
                break;
            }
        }

        out
    }
}

impl_passthrough_op_1!(SphereTraceLipschitzOp<MAX_STEPS>, Distance<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceLipschitzOp<MAX_STEPS>, Normal<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceLipschitzOp<MAX_STEPS>, Tangent<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceLipschitzOp<MAX_STEPS>, Uv<Pos>, Pos, const MAX_STEPS: u32);
impl_passthrough_op_1!(SphereTraceLipschitzOp<MAX_STEPS>, Color<Pos>, Pos, const MAX_STEPS: u32);

pub type SphereTraceLipschitz<const MAX_STEPS: u32, Sdf> =
    Operator<SphereTraceLipschitzOp<MAX_STEPS>, Sdf>;

impl<const MAX_STEPS: u32, Sdf> SphereTraceLipschitz<MAX_STEPS, Sdf> {
    pub fn epsilon(&mut self) -> &mut f32 {
        self.op().epsilon()
    }

    pub fn frac_1_k(&mut self) -> &mut f32 {
        self.op().frac_1_k()
    }
}
