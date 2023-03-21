use core::{marker::PhantomData, ops::RangeInclusive};

use rust_gpu_bridge::{
    glam::{Vec2, Vec3},
    Abs,
};
use type_fields::field::Field;

use crate::{
    default,
    signed_distance_field::{
        adapters::normals::CentralDiffNormal,
        attributes::{distance::Distance, normal::Normal},
        DistanceFunction,
    },
};

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
            samples: -10..=10,
            step: 2.0 / 20.0,
            epsilon: 0.5,
            _phantom: default(),
        }
    }
}

impl<Dim, Sdf> BoundChecker<Dim, Sdf> {
    const DERIV_EPSILON: f32 = 0.000005;
}

impl<Sdf> BoundChecker<Vec2, Sdf>
where
    Sdf: DistanceFunction<Vec2, Distance> + Clone + 'static,
{
    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                // Create sample coordinate
                let pos = Vec2::new(x as f32, y as f32) * self.step;

                // Calculate normal
                let normal: Normal<Vec2> =
                    CentralDiffNormal::<Sdf>::new(self.sdf.clone(), self.step)
                        .with(CentralDiffNormal::epsilon, self.epsilon)
                        .evaluate(pos);
                let normal = *normal;

                // Apply 1D central differencing along normal,
                // resulting in distance-space derivative
                let a = *self.sdf.evaluate(pos - normal * self.epsilon);
                let b = *self.sdf.evaluate(pos + normal * self.epsilon);
                let deriv = b - a;

                // Assert that derivative is 1 (w.r.t. floating-point error)
                if deriv.abs() - (self.epsilon * 2.0) > Self::DERIV_EPSILON {
                    //panic!("{deriv:?} at position {pos:?} is non-unit, resulting in a bound.");
                    return true;
                }
            }
        }

        false
    }
}

impl<Sdf> BoundChecker<Vec3, Sdf>
where
    Sdf: DistanceFunction<Vec3, Distance> + Clone + 'static,
{
    pub fn is_field(self) -> bool {
        !self.is_bound()
    }

    pub fn is_bound(self) -> bool {
        // Iterate over a regular grid
        for x in self.samples.clone() {
            for y in self.samples.clone() {
                for z in self.samples.clone() {
                    // Create sample coordinate
                    let pos = Vec3::new(x as f32, y as f32, z as f32) * self.step;

                    // Calculate normal
                    let normal: Normal<Vec3> =
                        CentralDiffNormal::<Sdf>::new(self.sdf.clone(), self.step)
                            .with(CentralDiffNormal::epsilon, self.epsilon)
                            .evaluate(pos);
                    let normal = *normal;

                    // Apply 1D central differencing along normal,
                    // resulting in distance-space derivative
                    let a = *self.sdf.evaluate(pos - normal * self.epsilon);
                    let b = *self.sdf.evaluate(pos + normal * self.epsilon);
                    let deriv = b - a;

                    // Assert that derivative is 1 (w.r.t. floating-point error)
                    if deriv.abs() - (self.epsilon * 2.0) > Self::DERIV_EPSILON {
                        //panic!("{deriv:?} at position {pos:?} is non-unit, resulting in a bound.");
                        return true;
                    }
                }
            }
        }

        false
    }
}
