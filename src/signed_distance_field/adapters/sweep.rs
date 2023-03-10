//! Create a 3D distance field by sweeping a 2D distance field
//! around the perimiter of another 2D distance field

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::signed_distance_field::SignedDistanceField;

/// Create a 3D distance field by sweeping a 2D distance field
/// around the perimiter of another 2D distance field
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sweep<SdfA, SdfB>
where
    SdfA: SignedDistanceField<Vec2>,
    SdfB: SignedDistanceField<Vec2>,
{
    pub sdf_a: SdfA,
    pub sdf_b: SdfB,
}

impl<SdfA, SdfB> SignedDistanceField<Vec3> for Sweep<SdfA, SdfB>
where
    SdfA: SignedDistanceField<Vec2>,
    SdfB: SignedDistanceField<Vec2>,
{
    fn distance(&self, p: Vec3) -> f32 {
        let q = Vec2::new(self.sdf_a.distance(p.truncate()), p.z);
        self.sdf_b.distance(q)
    }
}

