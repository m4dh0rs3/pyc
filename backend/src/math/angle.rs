// generic because I can. f32 would suffice, since new wn algo is solid

/// Holds an value of radians. This is not continuos `[0,π]`, but a value of [`y.atan2(x)`](https://doc.rust-lang.org/std/primitive.f64.html#method.atan2).
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle<T>(pub T);

impl<T> Angle<T> {
    /// Returns a new angle.
    fn new(angle: T) -> Self {
        Self(angle)
    }
}

impl<T> From<T> for Angle<T> {
    fn from(angle: T) -> Self {
        Angle(angle)
    }
}

// # deref
// mainly for trigonometric functions of [`f32`] and [`f64`]

use std::ops;

impl<T> ops::Deref for Angle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// # trigonometry

use super::vec_2d::Vec2D;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

macro_rules! angle_trig {
    ($Float: ty) => {
        impl Angle<$Float> {
            /// Returns a zero (0deg) angle.
            pub fn zero() -> Self {
                Self(0 as $Float)
            }

            /// Returns a right (90deg) angle.
            pub fn quarter() -> Self {
                Self(FRAC_PI_2 as $Float)
            }

            /// Returns a straight (180deg) angle.
            pub fn straight() -> Self {
                Self(PI as $Float)
            }

            /// Returns a three quarter (270deg) angle.
            pub fn three_quarter() -> Self {
                Self((PI + FRAC_PI_2) as $Float)
            }

            /// Returns a full (360deg) angle.
            pub fn full() -> Self {
                Self(TAU as $Float)
            }

            /// Get [`Angle<T>`] from [`Vec2D<T>`].
            pub fn from_vec_2d(vec_2d: Vec2D<$Float>) -> Self {
                vec_2d.angle()
            }
        }

        impl From<Vec2D<$Float>> for Angle<$Float> {
            fn from(vec_2d: Vec2D<$Float>) -> Self {
                vec_2d.angle()
            }
        }

        impl ops::Add for Angle<$Float> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl ops::Sub for Angle<$Float> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl ops::Mul<$Float> for Angle<$Float> {
            type Output = Self;

            fn mul(self, rhs: $Float) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl ops::Div<$Float> for Angle<$Float> {
            type Output = Self;

            fn div(self, rhs: $Float) -> Self::Output {
                Self(self.0 / rhs)
            }
        }
    };
}

// angle_trig!(f64);
angle_trig!(f32);
