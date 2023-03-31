//! Analytical raytracer.

use rust_gpu_bridge::glam::Vec3;
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        Color, Distance, FieldOperator, Normal, Operator, RaycastInput, RaycastOutput, Tangent, Uv,
    },
};

/// Compute the intersection between self and the given ray
pub trait RayIntersection {
    fn intersect(&self, eye: Vec3, dir: Vec3) -> Option<f32>;
}

/// Analytical raytracer.
///
/// Evaluates the [`RayIntersection`] of the provided type.
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct RaytraceOp;

impl<Sdf> FieldOperator<Sdf, RaycastInput, RaycastOutput> for RaytraceOp
where
    Sdf: RayIntersection,
{
    fn operator(
        &self,
        mut out: RaycastOutput,
        sdf: &Sdf,
        input: RaycastInput,
    ) -> <RaycastOutput as crate::prelude::Attribute>::Type {
        out.steps = 1;

        if let Some(t) = sdf.intersect(input.eye + input.dir * input.start, input.dir) {
            out.hit = true;
            out.closest_t = t;
            out.closest_dist = 0.0;
        }

        out
    }
}

impl_passthrough_op_1!(RaytraceOp, Distance, Pos,);
impl_passthrough_op_1!(RaytraceOp, Normal<Pos>, Pos,);
impl_passthrough_op_1!(RaytraceOp, Tangent<Pos>, Pos,);
impl_passthrough_op_1!(RaytraceOp, Uv, Pos,);
impl_passthrough_op_1!(RaytraceOp, Color, Pos,);

pub type Raytrace<Sdf> = Operator<RaytraceOp, Sdf>;
