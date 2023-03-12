//! Create a 3D distance field by sweeping a 2D distance field
//! around the perimiter of another 2D distance field

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::SignedDistanceField;

/// Create a 3D distance field by sweeping a 2D distance field
/// around the perimiter of another 2D distance field
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct Sweep<Core, Shell>
where
    Core: SignedDistanceField<Vec2, f32>,
    Shell: SignedDistanceField<Vec2, f32>,
{
    pub core: Core,
    pub shell: Shell,
}

impl<Core, Shell> SignedDistanceField<Vec3, f32> for Sweep<Core, Shell>
where
    Core: SignedDistanceField<Vec2, f32>,
    Shell: SignedDistanceField<Vec2, f32>,
{
    fn evaluate(&self, p: Vec3) -> f32 {
        let q = Vec2::new(self.core.evaluate(p.truncate()), p.z);
        self.shell.evaluate(q)
    }
}

#[cfg(test)]
pub mod tests {
    use type_fields::field::Field;

    use crate::signed_distance_field::shapes::composite::Circle;

    use super::Sweep;

    #[test]
    fn test_sweep() {
        Sweep::<Circle, Circle>::default()
            .with(Sweep::<(), ()>::CORE, Circle::default())
            .with(Sweep::<(), ()>::SHELL, Circle::default());
    }
}
