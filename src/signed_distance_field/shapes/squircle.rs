use rust_gpu_bridge::prelude::{Vec2, Vec2Swizzles};

use crate::signed_distance_field::{attributes::distance::Distance, SignedDistanceField};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Squircle;

impl SignedDistanceField<Vec2, Distance> for Squircle {
    fn evaluate(&self, mut p: Vec2) -> Distance {
        // ReflectAxial
        p = p.abs();

        // Cheap diagonal mirror
        if p.y > p.x {
            p = p.yx()
        }

        let a = p.x - p.y;
        let b = p.x + p.y;
        let c = (2.0 * b - 1.0) / 3.0;
        let h = a * a + c * c * c;

        let t = if h >= 0.0 {
            let h = h.sqrt();
            (h - a).signum() * (h - a).abs().powf(1.0 / 3.0) - (h + a).powf(1.0 / 3.0)
        } else {
            let z = (-c).sqrt();
            let v = (a / (c * z)).acos() / 3.0;
            -z * (v.cos() + v.sin() * 1.732050808)
        } * 0.5;

        let w = Vec2::new(-t, t) + 0.75 - t * t - p;

        (w.length() * (a * a * 0.5 + b - 1.5).signum()).into()
    }
}

