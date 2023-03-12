use rust_gpu_bridge::prelude::Vec3;

use crate::prelude::{Capsule, Distance, Line, Point, SignedDistanceField, Sphere};

// Sample a SDF several times along the provided axis, asserting expected distance output at each step
fn sample_axis_3d<Sdf: SignedDistanceField<Vec3, Distance>>(
    sdf: Sdf,
    axis: Vec3,
    expected: &[(f32, f32)],
) {
    for (sample_dist, expected) in expected {
        let dist = sdf.evaluate(axis * *sample_dist);
        assert_eq!(
            *dist, *expected,
            "Sample at dist {sample_dist:}, expected: {expected:}, actual: {dist:}"
        );
    }
}

#[test]
fn test_point() {
    sample_axis_3d(Point::default(), Vec3::X, &[(0.0, 0.0), (1.0, 1.0)]);
}

#[test]
fn test_line() {
    sample_axis_3d(
        Line::default(),
        Vec3::X,
        &[(0.0, 0.0), (1.0, 0.0), (2.0, 1.0)],
    );
}

#[test]
fn test_sphere() {
    let sdf = Sphere::default();

    sample_axis_3d(sdf, Vec3::X, &[(0.0, -1.0), (1.0, 0.0), (2.0, 1.0)])
}

#[test]
fn test_capsule() {
    let sdf = Capsule::default();

    sample_axis_3d(
        sdf,
        Vec3::X,
        &[(0.0, -1.0), (1.0, -1.0), (2.0, 0.0), (3.0, 1.0)],
    );
}
