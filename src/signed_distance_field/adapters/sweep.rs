//! Create a 3D distance field by sweeping a 2D distance field
//! around the perimiter of another 2D distance field

use rust_gpu_bridge::prelude::{Vec2, Vec3};
use type_fields::Field;

use crate::signed_distance_field::{Distance, SignedDistanceField};

/// Create a 3D distance field by sweeping a 2D distance field
/// around the perimiter of another 2D distance field
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Field)]
pub struct Sweep<Core, Shell> {
    pub core: Core,
    pub shell: Shell,
}

impl<Core, Shell> SignedDistanceField<Vec2, Distance> for Sweep<Core, Shell>
where
    Core: SignedDistanceField<f32, Distance>,
    Shell: SignedDistanceField<f32, Distance>,
{
    fn evaluate(&self, p: Vec2) -> Distance {
        let q = self.core.evaluate(p.x);
        self.shell.evaluate(*q)
    }
}

impl<Core, Shell> SignedDistanceField<Vec3, Distance> for Sweep<Core, Shell>
where
    Core: SignedDistanceField<Vec2, Distance>,
    Shell: SignedDistanceField<Vec2, Distance>,
{
    fn evaluate(&self, p: Vec3) -> Distance {
        let q = Vec2::new(*self.core.evaluate(p.truncate()), p.z);
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
            .with(Sweep::core, Circle::default())
            .with(Sweep::shell, Circle::default());
    }
}
