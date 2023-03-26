//! Extrude a 2D distance field into 3D.

#[cfg(not(feature = "spirv-std"))]
use core::fmt::Debug;

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::Field;

use crate::prelude::{Distance, FieldFunction};

/// Extrude a 2D distance field into 3D.
#[derive(Default, Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct Extrude<Sdf> {
    pub sdf: Sdf,
    pub axis: Vec3,
    pub depth: f32,
}

#[cfg(not(feature = "spirv-std"))]
impl<Sdf> Debug for Extrude<Sdf>
where
    Sdf: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.sdf.fmt(f)?;
        self.axis.fmt(f)?;
        self.depth.fmt(f)?;
        Ok(())
    }
}

impl<Sdf> FieldFunction<Vec2, Distance> for Extrude<Sdf>
where
    Sdf: FieldFunction<f32, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Vec2) -> f32 {
        let d = self.sdf.evaluate(attr, p.x);
        let w = Vec2::new(d, p.y.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

impl<Sdf> FieldFunction<Vec3, Distance> for Extrude<Sdf>
where
    Sdf: FieldFunction<Vec2, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Vec3) -> f32 {
        let d = self.sdf.evaluate(attr, p.truncate());
        let w = Vec2::new(d, p.z.abs() - self.depth);
        w.x.max(w.y).min(0.0) + w.max(Vec2::ZERO).length()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use rust_gpu_bridge::glam::Vec3;

    use crate::{prelude::BoundChecker, signed_distance_field::shapes::composite::Square};

    use super::Extrude;

    #[test]
    fn test_extrude() {
        assert!(BoundChecker::<Vec3, Extrude::<Square>>::default().is_field());
    }
}
