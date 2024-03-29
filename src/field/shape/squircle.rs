use rust_gpu_bridge::{
    glam::{Vec2, Vec2Swizzles},
    Abs, Acos, Cos, Pow, Sign, Sin, Sqrt,
};

use crate::prelude::{AttrDistance, Field, items::position::Position, Distance};

// Inigo Quilez' quadratic circle
// Appears to be a superellipse / lame curve with n = 1.0 / 0.75
// Can be generalized to 3D as a superellipsoid
//
// Desmos decomposition: https://www.desmos.com/calculator/i9cgthn0ls
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct Squircle;

impl Field<AttrDistance<Vec2>> for Squircle {
    fn field(&self, p: &Position<Vec2>) -> Distance {
        // Axial reflection
        let mut p = p.abs();

        // Cheap diagonal mirror
        if p.y > p.x {
            p = p.yx()
        }

        // Diagonal X maximum
        let a = p.x - p.y;

        // Diagonal Y minimum
        let b = p.x + p.y;

        // Diagonal Y maximum
        let c = (2.0 * b - 1.0) / 3.0;

        // Semicircle at (0.5, 0.5)
        let h = a * a + c * c * c;

        let t = if h >= 0.0 {
            // Appears identical to h in graph plot, maybe field related
            let h = h.sqrt();
            // Uneven circular curve
            (h - a).sign() * (h - a).abs().pow(1.0 / 3.0) - (h + a).pow(1.0 / 3.0)
        } else {
            // Negative Y minimum
            let z = (-c).sqrt();
            // Uneven tangent curve
            let v = (a / (c * z)).acos() / 3.0;
            // Uneven tangent curve, repeating w/different frequency
            -z * (v.cos() + v.sin() * 1.732050808)
        } * 0.5;

        // Bounded quadradic curve
        let w = Vec2::new(-t, t) + 0.75 - t * t - p;

        // Quadratic curve sign
        let s = (a * a * 0.5 + b - 1.5).sign();

        // Final curve w / sign
        (w.length() * s).into()
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod test {
    use crate::prelude::{BoundTester, NormalTetrahedron};

    use super::Squircle;

    #[test]
    pub fn test_squircle() {
        assert!(BoundTester::<NormalTetrahedron::<Squircle>>::default().is_field_2d())
    }
}
