//! Analytical raytracer.

use rust_gpu_bridge::glam::Vec3;
use type_fields::Field;

use crate::{
    impl_passthrough_op_1,
    prelude::{
        AttrColor, AttrDistance, FieldOperator, AttrNormal, Operator, Raycast, RaycastInput, RaycastOutput,
        AttrTangent, AttrUv,
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
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct RaytraceOp;

impl<Sdf> FieldOperator<Sdf, Raycast> for RaytraceOp
where
    Sdf: RayIntersection,
{
    fn operator(
        &self,
        sdf: &Sdf,
        input: &RaycastInput,
    ) -> <Raycast as crate::prelude::Attribute>::Output {
        let mut out = RaycastOutput::default();

        out.steps = 1;

        if let Some(t) = sdf.intersect(input.eye + input.dir * input.start, input.dir) {
            out.closest_t = t;
            out.closest_dist = 0.0;
        }

        out
    }
}

impl_passthrough_op_1!(RaytraceOp, AttrDistance<Pos>, Pos,);
impl_passthrough_op_1!(RaytraceOp, AttrNormal<Pos>, Pos,);
impl_passthrough_op_1!(RaytraceOp, AttrTangent<Pos>, Pos,);
impl_passthrough_op_1!(RaytraceOp, AttrUv<Pos>, Pos,);
impl_passthrough_op_1!(RaytraceOp, AttrColor<Pos>, Pos,);

pub type Raytrace<Sdf> = Operator<RaytraceOp, Sdf>;
