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

#[allow(non_camel_case_types)]
type Ball_Radius = crate::operators::isosurface::Isosurface_Delta;

impl Ball {
    pub const RADIUS: Ball_Radius = Isosurface::<()>::DELTA;
}

/// A 2D circle.
pub type Circle = Ball;

#[allow(non_camel_case_types)]
pub type Circle_Radius = crate::operators::isosurface::Isosurface_Delta;

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
        Circle::default().with(Circle::RADIUS, f32::default());
    }

    #[test]
    fn test_sphere() {
        Sphere::default().with(Sphere::RADIUS, f32::default());
    }
}
