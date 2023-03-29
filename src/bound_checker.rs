use core::{marker::PhantomData, ops::RangeInclusive};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::field::Field;

use crate::prelude::{default, Distance, FieldFunction, Normal, NormalCentralDiff};

/// Asserts that the provided distance function is a field rather than a bound
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoundChecker<Dim, Sdf> {
    pub sdf: Sdf,
    pub samples: RangeInclusive<isize>,
    pub step: f32,
    pub epsilon: f32,
    pub _phantom: PhantomData<Dim>,
}

impl<Dim, Sdf> Default for BoundChecker<Dim, Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        BoundChecker {
            sdf: default(),
            samples: -20..=20,
            step: 10.0 / 20.0,
            epsilon: 0.5,
            _phantom: default(),
        }
    }
}

impl<Sdf> BoundChecker<Vec2, Sdf>
where
    Sdf: FieldFunction<Vec2, Distance>
        + FieldFunction<Vec2, Normal<Vec2>>
        + Default
        + Clone
        + 'static,
{
    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                // Compose sample coordinate
                let pos = Vec2::new(x as f32, y as f32) * self.step;

                // Calculate normal
                let n = self.sdf.evaluate(Normal::<Vec2>::default(), pos);

                // Skip samples where normal is not valid
                // (ex. the center of a sphere)
                if !n.is_normalized() {
                    continue;
                }

                // Calculate distance
                let d = self.sdf.evaluate(Distance, pos);

                // Calculate vector from position to nearest surface
                let surface = n * -d;

                // Evaluate distance at surface vector, asserting that it is near zero
                let r = self.sdf.evaluate(Distance, pos + surface);
                assert!(
                    r.abs() <= 0.00001,
                    "Encountered reciprocal distance {r:} at point {:}, {:} with distance {d:} and normal {:}, {}",
                    pos.x,
                    pos.y,
                    n.x,
                    n.y
                );
            }
        }

        false
    }
}

impl<Sdf> BoundChecker<Vec3, Sdf>
where
    Sdf: FieldFunction<Vec3, Distance>
        + FieldFunction<Vec3, Normal<Vec3>>
        + Default
        + Clone
        + 'static,
{
    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                for z in self.samples.clone() {
                    // Compose sample coordinate
                    let pos = Vec3::new(x as f32, y as f32, z as f32) * self.step;

                    // Calculate normal
                    let n = self.sdf.evaluate(Normal::<Vec3>::default(), pos);

                    // Skip samples where normal is not valid
                    // (ex. the center of a sphere)
                    if !n.is_normalized() {
                        continue;
                    }

                    // Calculate distance
                    let d = self.sdf.evaluate(Distance, pos);

                    // Calculate vector from position to nearest surface
                    let surface = n * -d;

                    // Evaluate distance at surface vector, asserting that it is near zero
                    let r = self.sdf.evaluate(Distance, pos + surface);
                    assert!(
                        r.abs() <= 0.00001,
                        "Encountered reciprocal distance {r:} at point {:}, {:}, {:} with distance {d:} and normal {:}, {:}, {:}",
                        pos.x,
                        pos.y,
                        pos.z,
                        n.x,
                        n.y,
                        n.z
                    );
                }
            }
        }

        false
    }
}
