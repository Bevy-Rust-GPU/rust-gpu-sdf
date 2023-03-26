//! Create a 3D distance field by sweeping a 2D distance field
//! around the perimiter of another 2D distance field

use rust_gpu_bridge::glam::{Vec2, Vec3, Vec3Swizzles};
use type_fields::Field;

use crate::prelude::{default, Distance, FieldFunction, Normal, Uv};

/// Create a 3D distance field by sweeping a 2D distance field
/// around the perimiter of another 2D distance field
#[derive(Copy, Clone, PartialEq, Field)]
#[repr(C)]
pub struct Sweep<Core, Shell> {
    pub core: Core,
    pub shell: Shell,
}

impl<Core, Shell> Default for Sweep<Core, Shell>
where
    Core: Default,
    Shell: Default,
{
    fn default() -> Self {
        Sweep {
            core: default(),
            shell: default(),
        }
    }
}

impl<Core, Shell> FieldFunction<Vec2, Distance> for Sweep<Core, Shell>
where
    Core: FieldFunction<f32, Distance>,
    Shell: FieldFunction<f32, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Vec2) -> f32 {
        let q = self.core.evaluate(attr, p.x);
        self.shell.evaluate(attr, q)
    }
}

impl<Core, Shell> FieldFunction<Vec3, Distance> for Sweep<Core, Shell>
where
    Core: FieldFunction<Vec2, Distance>,
    Shell: FieldFunction<Vec2, Distance>,
{
    fn evaluate(&self, attr: Distance, p: Vec3) -> f32 {
        let q = Vec2::new(self.core.evaluate(attr, p.truncate()), p.z);
        self.shell.evaluate(attr, q)
    }
}

impl<Core, Shell> FieldFunction<Vec3, Normal<Vec3>> for Sweep<Core, Shell>
where
    Core: FieldFunction<Vec2, Distance>,
    Shell: FieldFunction<Vec2, Normal<Vec2>>,
{
    fn evaluate(&self, _attr: Normal<Vec3>, p: Vec3) -> Vec3 {
        let q = Vec2::new(self.core.evaluate(Distance, p.truncate()), p.z);
        let n = self.shell.evaluate(Normal::<Vec2>::default(), q);
        let w = p.xy().normalize() * n.x;
        Vec3::new(w.x, w.y, n.y).into()
    }
}

impl<Core, Shell> FieldFunction<Vec3, Uv> for Sweep<Core, Shell>
where
    Core: FieldFunction<Vec2, Distance> + FieldFunction<Vec2, Uv>,
    Shell: FieldFunction<Vec2, Uv>,
{
    fn evaluate(&self, attr: Uv, p: Vec3) -> Vec2 {
        let dist_core = self.core.evaluate(Distance, p.truncate());
        let uv_core = self.core.evaluate(attr, p.truncate());
        let q = Vec2::new(dist_core, p.z);
        let uv_shell = self.shell.evaluate(attr, q);
        Vec2::new(
            uv_core.x,
            uv_shell.x + uv_shell.y,
        )
        .into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use rust_gpu_bridge::glam::Vec3;

    use crate::{prelude::BoundChecker, signed_distance_field::shapes::composite::Circle};

    use super::Sweep;

    #[test]
    fn test_sweep() {
        assert!(BoundChecker::<Vec3, Sweep::<Circle, Circle>>::default().is_field());
    }
}
