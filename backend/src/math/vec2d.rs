/// 2-Dimensional vector of `T` on `x` and `y`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vec2D<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Vec2D<T> {
    /// Creates new [`Vec2D`].
    pub(crate) fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::ops;

impl<T: ops::Add<Output = T>> ops::Add for Vec2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add<T> for Vec2D<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: ops::AddAssign> ops::AddAssign for Vec2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: ops::AddAssign + Copy> ops::AddAssign<T> for Vec2D<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Vec2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub<T> for Vec2D<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Vec2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: ops::SubAssign + Copy> ops::SubAssign<T> for Vec2D<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T: ops::Mul<Output = T>> ops::Mul for Vec2D<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: ops::MulAssign> ops::MulAssign for Vec2D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vec2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: ops::MulAssign + Copy> ops::MulAssign<T> for Vec2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/* impl ops::Mul<Vec2D<f64>> for f64 {
    type Output = Vec2D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
} */

impl<T: ops::Div<Output = T>> ops::Div for Vec2D<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Vec2D<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: ops::DivAssign + Copy> ops::DivAssign<T> for Vec2D<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/* impl ops::Div<Vec2D<f64>> for f64 {
    type Output = Vec2D<f64>;

    fn div(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
} */

use super::turn::Turn;

macro_rules! vec2d_trig {
    ($Float: ty) => {
        impl Vec2D<$Float> {
            /// Returns the length of the vector.
            pub(crate) fn maq(&self) -> $Float {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }

            /// Returns the normal of the vector.
            fn normal(&mut self) -> Self {
                let maq = self.maq();

                Self {
                    x: self.x / maq,
                    y: self.y / maq,
                }
            }

            /// Returns a vector, rounded to integers.
            fn round(&self) -> Self {
                Self {
                    x: self.x.round(),
                    y: self.y.round(),
                }
            }

            /// Creates a vector from polar coordinates.
            pub(crate) fn from_polar(turn: Turn, radius: $Float) -> Self {
                Self {
                    x: radius * turn.cos() as $Float,
                    y: radius * turn.sin() as $Float,
                }
            }

            /// Returns turns around the origin.
            pub(crate) fn turn(&self) -> Turn {
                (Turn::from_pi(self.y.atan2(-self.x).into())).normal()
            }
        }
    };
}

//vec2d_trig!(f32);
vec2d_trig!(f64);

use super::utils::{bezier, lerp};

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Vec2D<T> {
    /// Interpolates linearly from `a` to `b` given t in `[0; 1]`.
    fn lerp(t: T, a: Self, b: Self) -> Self {
        Self {
            x: lerp(t, a.x, b.x),
            y: lerp(t, a.y, b.y),
        }
    }

    /// Interpolates linearly between `a`, `b` and `c` given t in `[0; 1]`.
    fn bezier(t: T, a: Self, b: Self, c: Self) -> Self {
        Self {
            x: bezier(t, a.x, b.x, c.x),
            y: bezier(t, a.y, b.y, c.y),
        }
    }
}

impl<
        T: ops::Mul<Output = T>
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + PartialEq
            + PartialOrd
            + From<i8>
            + Copy,
    > Vec2D<T>
{
    /// Returns the 2-dimensional cross product of the vector.
    fn cross_zero(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Returns the point of intersection of to line segments.
    fn intersect(p1: Self, p2: Self, r1: Self, r2: Self) -> Option<Self> {
        let s1 = p2 - p1;
        let s2 = r2 - r1;

        let k = s1.cross_zero(&s2);

        if k == 0.into() {
            return None;
        }

        let d = p1 - r1;

        let s = s1.cross_zero(&d) / k;
        let t = s2.cross_zero(&d) / k;

        if s > 0.into() && s < 1.into() && t > 0.into() && t < 1.into() {
            Some(Self {
                x: p1.x + t * s1.x,
                y: p1.y + t * s1.y,
            })
        } else {
            None
        }
    }
}

macro_rules! from_float {
    ($Float: ty, $Integral: ty) => {
        impl From<Vec2D<$Float>> for Vec2D<$Integral> {
            fn from(vec_2d: Vec2D<$Float>) -> Self {
                Self {
                    x: vec_2d.x.round() as $Integral,
                    y: vec_2d.y.round() as $Integral,
                }
            }
        }
    };
}

macro_rules! from_integral {
    ($Integral: ty, $Float: ty) => {
        impl From<Vec2D<$Integral>> for Vec2D<$Float> {
            fn from(vec_2d: Vec2D<$Integral>) -> Self {
                Self {
                    x: vec_2d.x as $Float,
                    y: vec_2d.y as $Float,
                }
            }
        }
    };
}

from_integral!(f32, f64);
from_integral!(f64, f32);

from_float!(f64, i8);
from_float!(f64, i16);
from_float!(f64, i32);
from_float!(f64, i64);

from_float!(f32, i8);
from_float!(f32, i16);
from_float!(f32, i32);
from_float!(f32, i64);

from_integral!(i8, f64);
from_integral!(i16, f64);
from_integral!(i32, f64);
from_integral!(i64, f64);

from_integral!(i8, f32);
from_integral!(i16, f32);
from_integral!(i32, f32);
from_integral!(i64, f32);

impl<T: From<i8>> Vec2D<T> {
    /// Create new vector at origin.
    pub(crate) fn zero() -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
        }
    }
}
