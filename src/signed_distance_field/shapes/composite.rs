//! Shapes composed from other shapes.

use crate::{
    markers::Exact,
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
pub type Ball<Dim> = Isosurface<Point, Dim>;

/// A 2D circle.
pub type Circle = Ball<D2>;

/// A 3D sphere.
pub type Sphere = Ball<D3>;

/// A capsule.
pub type Capsule<Dim> = Operator<Line<Dim>, IsosurfaceOp, Dim>;

/// A box.
pub type Box<Dim> = Elongate<Point, Exact, Dim>;

/// A 2D square.
pub type Square = Box<D2>;

/// A 3D cube.
pub type Cube = Box<D3>;

/// A 3D torus.
pub type Torus = Sweep<Circle, Circle>;
