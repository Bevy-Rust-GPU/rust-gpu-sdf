//! Shapes composed from other shapes.

use crate::{
    markers::Exact,
    operators::{
        elongate::Elongate,
        extrude::ExtrudeDist,
        round::{Round, RoundOp},
        Operator,
    },
    signed_distance_field::metrics::euclidean::EuclideanMetric,
    D2, D3,
};

/// An infinitely small point.
/// Not very useful on its own; primarily used for composition.
pub type Point = EuclideanMetric;

/// An infinitely thin line.
/// Not very useful on its own; primarily used for composition.
pub type Line<Dim> = ExtrudeDist<Point, Dim>;

/// A ball.
pub type Ball<Dim> = Round<Point, Dim>;

/// A 2D circle.
pub type Circle = Ball<D2>;

/// A 3D sphere.
pub type Sphere = Ball<D3>;

/// A capsule.
pub type Capsule<Dim> = Operator<Line<Dim>, RoundOp, Dim>;

/// A box.
pub type Box<Dim> = Elongate<Point, Exact, Dim>;

/// A 2D square.
pub type Square = Box<D2>;

/// A 3D cube.
pub type Cube = Box<D3>;
