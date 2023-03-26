//! Attributes that can be computed by a distance field

pub trait Attribute {
    type Type;
}

pub mod distance {
    use super::Attribute;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct Distance;

    impl Attribute for Distance {
        type Type = f32;
    }
}

pub mod position {
    use rust_gpu_bridge::glam::Vec3;

    use super::Attribute;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct Position;

    impl Attribute for Position {
        type Type = Vec3;
    }
}

pub mod normal {
    use core::marker::PhantomData;

    use super::Attribute;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct Normal<Dim>(PhantomData<Dim>);

    impl<Dim> Clone for Normal<Dim> {
        fn clone(&self) -> Self {
            Normal(self.0.clone())
        }
    }

    impl<Dim> Copy for Normal<Dim> {}

    impl<Dim> Attribute for Normal<Dim> {
        type Type = Dim;
    }
}

pub mod uv {
    use rust_gpu_bridge::glam::Vec2;

    use super::Attribute;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct Uv;

    impl Attribute for Uv {
        type Type = Vec2;
    }
}

pub mod tangent {
    use core::marker::PhantomData;

    use super::Attribute;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct Tangent<Dim>(PhantomData<Dim>);

    impl<Dim> Attribute for Tangent<Dim> {
        type Type = Dim;
    }
}

pub mod color {
    use rust_gpu_bridge::glam::Vec4;

    use super::Attribute;

    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct Color;

    impl Attribute for Color {
        type Type = Vec4;
    }
}
