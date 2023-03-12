//! Shapes composed from other shapes.

use crate::{
    operators::{
        elongate::Elongate,
        isosurface::{Isosurface, IsosurfaceOp},
        stretch::StretchDist,
        Operator,
    },
    signed_distance_field::{adapters::sweep::Sweep, metrics::euclidean::EuclideanMetric},
    D2, D3,
};

/// An infinitely small point.
/// Not very useful on its own; primarily used for composition.
pub type Point = EuclideanMetric;

/// An infinitely thin line.
/// Not very useful on its own; primarily used for composition.
pub type Line<Dim> = StretchDist<Point, Dim>;

/// A ball.
pub type Ball = Isosurface<Point>;

impl Ball {
    pub fn radius(&mut self) -> &mut f32 {
        &mut self.op.delta
    }
}

/// A 2D circle.
pub type Circle = Ball;

/// A 3D sphere.
pub type Sphere = Ball;

/// A capsule.
pub type Capsule<Dim> = Operator<Line<Dim>, IsosurfaceOp>;

/// A box.
pub type Box<Dim> = Elongate<Point, Dim>;

/// A 2D square.
pub type Square = Box<D2>;

/// A 3D cube.
pub type Cube = Box<D3>;

/// A 3D torus.
pub type Torus = Sweep<Circle, Circle>;

#[cfg(test)]
pub mod tests {
    use type_fields::field::Field;

    use super::{Circle, Sphere};

    #[test]
    fn test_circle() {
        Circle::default().with(Circle::radius, f32::default());
    }

    #[test]
    fn test_sphere() {
        Sphere::default().with(Sphere::radius, f32::default());
    }
}
