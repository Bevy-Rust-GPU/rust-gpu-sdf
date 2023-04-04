#![no_std]

pub mod bound_tester;
pub use type_fields;
pub mod attribute;
pub mod context;
pub mod field;

pub mod util {
    #[macro_export]
    macro_rules! impl_newtype {
        (#[derive($($derive:ident),*)] pub struct $ident:ident($ty:ident);) => {
            #[derive( $($derive),* )]
            pub struct $ident(pub $ty);

            crate::impl_morphisms_newtype!($ident($ty));
            crate::impl_arith_newtype!($ident($ty));
        };

        (#[derive($($derive:ident),*)] pub struct $ident:ident<$ty:ident>($ty2:ident);) => {
            #[derive( $($derive),* )]
            pub struct $ident<$ty>(pub $ty);

            crate::impl_morphisms_newtype!($ident<$ty>);
            crate::impl_arith_newtype!($ident<$ty>);
        };
    }

    #[macro_export]
    macro_rules! impl_morphisms_newtype {
        ($ident:ident($ty:ident)) => {
            impl Deref for $ident {
                type Target = $ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl DerefMut for $ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl From<$ty> for $ident {
                fn from(value: $ty) -> Self {
                    $ident(value)
                }
            }
        };
        ($ident:ident<$ty:ident>) => {
            impl<$ty> Deref for $ident<$ty> {
                type Target = $ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<$ty> DerefMut for $ident<$ty> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<$ty> From<$ty> for $ident<$ty> {
                fn from(value: $ty) -> Self {
                    $ident(value)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_arith_newtype {
        ($ident:ident ($ty:ident)) => {
            crate::impl_arith_newtype_1!(Neg, neg, $ident($ty));
            crate::impl_arith_newtype_2!(Add, add, $ident($ty));
            crate::impl_arith_newtype_2!(Sub, sub, $ident($ty));
            crate::impl_arith_newtype_2!(Mul, mul, $ident($ty));
            crate::impl_arith_newtype_2!(Div, div, $ident($ty));
        };
        ($ident:ident <$ty:ident>) => {
            crate::impl_arith_newtype_1!(Neg, neg, $ident<$ty>);
            crate::impl_arith_newtype_2!(Add, add, $ident<$ty>);
            crate::impl_arith_newtype_2!(Sub, sub, $ident<$ty>);
            crate::impl_arith_newtype_2!(Mul, mul, $ident<$ty>);
            crate::impl_arith_newtype_2!(Div, div, $ident<$ty>);
        };
    }

    #[macro_export]
    macro_rules! impl_arith_newtype_1 {
        ($op:ident, $fn:ident, $ident:ident ($ty:ident)) => {
            impl core::ops::$op for $ident {
                type Output = Self;

                fn $fn(self) -> Self::Output {
                    $ident(self.0.$fn())
                }
            }
        };
        ($op:ident, $fn:ident, $ident:ident <$ty:ident>) => {
            impl<$ty> core::ops::$op for $ident<$ty>
            where
                $ty: core::ops::$op<Output = $ty>,
            {
                type Output = Self;

                fn $fn(self) -> Self::Output {
                    $ident(self.0.$fn())
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_arith_newtype_2 {
        ($op:ident, $fn:ident, $ident:ident ($ty:ident)) => {
            impl<RHS> core::ops::$op<RHS> for $ident
            where
                $ty: core::ops::$op<RHS, Output = $ty>,
            {
                type Output = Self;

                fn $fn(self, rhs: RHS) -> Self::Output {
                    Self(self.0.$fn(rhs))
                }
            }
        };
        ($op:ident, $fn:ident, $ident:ident <$ty:ident>) => {
            impl<$ty, RHS> core::ops::$op<RHS> for $ident<$ty>
            where
                $ty: core::ops::$op<RHS, Output = $ty>,
            {
                type Output = Self;

                fn $fn(self, rhs: RHS) -> Self::Output {
                    Self(self.0.$fn(rhs))
                }
            }
        };
    }
}

pub mod prelude;

use rust_gpu_bridge::glam::{Vec2, Vec3};

pub type D1 = f32;
pub type D2 = Vec2;
pub type D3 = Vec3;

/// Free-standing [`Default::default()`] invocation
pub fn default<T: Default>() -> T {
    Default::default()
}
