use core::{marker::PhantomData, ops::RangeInclusive};

use rust_gpu_bridge::prelude::{Vec2, Vec3};

use crate::{
    default,
    signed_distance_field::{
        adapters::normals::CentralDiffNormal, attributes::distance::Distance, SignedDistanceField,
    },
};

/// Asserts that the provided distance function is a field rather than a bound
#[derive(Debug, Clone, PartialEq)]
pub struct BoundChecker<Dim, Sdf> {
    pub sdf: Sdf,
    pub samples: RangeInclusive<isize>,
    pub _phantom: PhantomData<Dim>,
}

impl<Dim, Sdf> Default for BoundChecker<Dim, Sdf>
where
    Sdf: Default,
{
    fn default() -> Self {
        BoundChecker {
            sdf: default(),
            samples: -10..=10,
            _phantom: default(),
        }
    }
}

impl<Dim, Sdf> BoundChecker<Dim, Sdf> {
    const DERIV_EPSILON: f32 = 0.000005;
}

impl<Sdf> BoundChecker<Vec2, Sdf>
where
    Sdf: SignedDistanceField<Vec2, Distance> + Clone,
{
    pub fn new(sdf: Sdf) -> Self {
        Self {
            sdf,
            samples: -10..=10,
            _phantom: default(),
        }
    }

    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                // Create sample coordinate
                let pos = Vec2::new(x as f32, y as f32) * 0.5;

                // Calculate normal
                let normal = *CentralDiffNormal::<Sdf>::new(self.sdf.clone(), 0.5).evaluate(pos);

                // Skip invalid normals (ex. crossing zero)
                if normal.is_nan() {
                    continue;
                }

                // Apply 1D central differencing along normal,
                // resulting in distance-space derivative
                let a = *self.sdf.evaluate(pos - normal * 0.5);
                let b = *self.sdf.evaluate(pos + normal * 0.5);
                let deriv = b - a;

                // Assert that derivative is 1 (w.r.t. floating-point error)
                if deriv.abs() - 1.0 > Self::DERIV_EPSILON {
                    // panic!("{deriv:?} at position {pos:?} is non-unit, resulting in a bound.");
                    return true;
                }
            }
        }

        false
    }
}

impl<Sdf> BoundChecker<Vec3, Sdf>
where
    Sdf: SignedDistanceField<Vec3, Distance> + Clone,
{
    pub fn new(sdf: Sdf) -> Self {
        Self {
            sdf,
            samples: -10..=10,
            _phantom: default(),
        }
    }

    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                for z in self.samples.clone() {
                    // Create sample coordinate
                    let pos = Vec3::new(x as f32, y as f32, z as f32) * 0.5;

                    // Calculate normal
                    let normal =
                        *CentralDiffNormal::<Sdf>::new(self.sdf.clone(), 0.5).evaluate(pos);

                    // Skip invalid normals (ex. crossing zero)
                    if normal.is_nan() {
                        continue;
                    }

                    // Apply 1D central differencing along normal,
                    // resulting in distance-space derivative
                    let a = *self.sdf.evaluate(pos - normal * 0.5);
                    let b = *self.sdf.evaluate(pos + normal * 0.5);
                    let deriv = b - a;

                    // Assert that derivative is 1 (w.r.t. floating-point error)
                    if deriv.abs() - 1.0 > Self::DERIV_EPSILON {
                        // panic!("{deriv:?} at position {pos:?} is non-unit, resulting in a bound.");
                        return true;
                    }
                }
            }
        }

        false
    }
}
